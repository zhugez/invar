#![forbid(unsafe_code)]

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("Invar v0.1 targets Linux and macOS only");

mod model;
mod runner;
mod storage;

use clap::{Parser, Subcommand};
use model::{CaseResult, Config, Error, Report, Result, Snapshot, TOTAL_OUTPUT_LIMIT};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "invar", version, about = "Change code. Preserve behavior.")]
struct Cli {
    #[arg(
        long,
        global = true,
        default_value = ".",
        help = "Project directory containing invar.json"
    )]
    project: PathBuf,
    #[arg(long, global = true, help = "Emit one versioned JSON report")]
    json: bool,
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(about = "Create a sample configuration without executing it")]
    Init,
    #[command(about = "Capture reviewed commands into a candidate, without approving it")]
    Capture,
    #[command(about = "Approve the exact candidate whose SHA-256 you reviewed")]
    Lock {
        #[arg(long, value_name = "SHA256")]
        candidate: String,
        #[arg(long, help = "Explicitly permit replacement of an accepted baseline")]
        replace: bool,
    },
    #[command(about = "Compare current behavior with accepted expectations, without writing them")]
    Verify,
}

impl Action {
    fn name(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Capture => "capture",
            Self::Lock { .. } => "lock",
            Self::Verify => "verify",
        }
    }
}

fn json_bytes(value: &impl serde::Serialize) -> Result<Vec<u8>> {
    let mut bytes =
        serde_json::to_vec_pretty(value).map_err(|e| Error::execution(e.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn parse_snapshot(bytes: &[u8], config: &Config) -> Result<Snapshot> {
    let snapshot: Snapshot = serde_json::from_slice(bytes)
        .map_err(|e| Error::baseline(format!("cannot parse snapshot: {e}")))?;
    snapshot.validate(config)?;
    Ok(snapshot)
}

fn run(cli: &Cli, report: &mut Report) -> Result<()> {
    let root = cli
        .project
        .canonicalize()
        .map_err(|e| Error::execution(format!("project: {e}")))?;
    if !root.is_dir() {
        return Err(Error::execution("project must be a directory"));
    }
    let io_error = |e: io::Error| Error::execution(e.to_string());
    if matches!(cli.action, Action::Init) {
        storage::init(&root, &json_bytes(&Config::sample())?).map_err(io_error)?;
        report.message = Some("Created invar.json and .invar. Review the configuration before capture. Nothing executed.".into());
        return Ok(());
    }
    let bytes =
        storage::read_config(&root).map_err(|e| Error::execution(format!("invar.json: {e}")))?;
    let config: Config =
        serde_json::from_slice(&bytes).map_err(|e| Error::execution(format!("invar.json: {e}")))?;
    config.validate().map_err(Error::execution)?;
    for case in &config.cases {
        model::working_dir(&root, case)?;
    }
    let state = storage::state(&root).map_err(|e| {
        if matches!(cli.action, Action::Verify | Action::Lock { .. }) {
            Error::baseline(format!(".invar: {e}"))
        } else {
            Error::execution(format!(".invar: {e}"))
        }
    })?;
    match &cli.action {
        Action::Init => unreachable!(),
        Action::Capture => {
            let _lock = storage::writer_lock(&state).map_err(io_error)?;
            let mut observations = Vec::new();
            let mut total = 0;
            for case in &config.cases {
                match runner::execute(&root, case) {
                    Ok(o) => {
                        total += o.stdout.len() + o.stderr.len();
                        if total > TOTAL_OUTPUT_LIMIT {
                            report.cases.push(CaseResult::error(
                                &case.id,
                                "aggregate snapshot output exceeds 8 MiB".into(),
                            ));
                        } else {
                            report.cases.push(CaseResult::new(&case.id, "captured"));
                            observations.push(o);
                        }
                    }
                    Err(e) => report.cases.push(CaseResult::error(&case.id, e.message)),
                }
            }
            if report.cases.iter().any(|c| c.status == "error") {
                return Err(Error::execution("capture failed; no candidate was published, and any previous candidate remains unchanged"));
            }
            let snapshot = Snapshot::new(config, observations);
            let bytes = json_bytes(&snapshot)?;
            if bytes.len() as u64 > model::SNAPSHOT_LIMIT {
                return Err(Error::execution("snapshot file size limit exceeded"));
            }
            storage::write_atomic(&state.join("candidate.json"), &bytes, true).map_err(io_error)?;
            report.candidate_sha256 = Some(model::digest(&bytes));
            report.message = Some("Candidate captured. Inspect .invar/candidate.json, then approve its exact hash with lock --candidate SHA256.".into());
        }
        Action::Lock { candidate, replace } => {
            let _lock = storage::writer_lock(&state).map_err(io_error)?;
            let bytes = storage::read_snapshot(&state, "candidate.json")
                .map_err(|e| Error::baseline(format!("candidate.json: {e}")))?;
            let hash = model::digest(&bytes);
            if candidate != &hash {
                return Err(Error::baseline("candidate SHA-256 does not match; review the current candidate before approval"));
            }
            parse_snapshot(&bytes, &config)?;
            storage::write_atomic(&state.join("accepted.json"), &bytes, *replace).map_err(|e| {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    Error::execution(
                        "accepted baseline exists; use --replace only after reviewing the change",
                    )
                } else {
                    io_error(e)
                }
            })?;
            report.candidate_sha256 = Some(hash);
            report.message = Some("Candidate explicitly approved. Accepted expectations are locally trusted; hashes are not authorization.".into());
        }
        Action::Verify => {
            let bytes = storage::read_snapshot(&state, "accepted.json")
                .map_err(|e| Error::baseline(format!("accepted.json: {e}")))?;
            let snapshot = parse_snapshot(&bytes, &config)?;
            for (case, expected) in config.cases.iter().zip(&snapshot.observations) {
                match runner::execute(&root, case) {
                    Ok(actual) => report.cases.push(model::compare(expected, &actual)),
                    Err(e) => report.cases.push(CaseResult::error(&case.id, e.message)),
                }
            }
            if report.cases.iter().any(|c| c.status == "error") {
                return Err(Error::execution(
                    "verification encountered execution errors; no successful verdict",
                ));
            }
            if report.cases.iter().any(|c| c.status == "regression") {
                report.exit_code = 1;
                report.status = "regression".into();
                report.message = Some("Behavior changed. Inspect the named channels and repair the code, or explicitly review a new candidate if the change is intended.".into());
            } else {
                report.status = "pass".into();
                report.message = Some("All configured examples match accepted behavior.".into());
            }
        }
    }
    Ok(())
}

fn escaped(text: &str) -> String {
    text.chars().flat_map(char::escape_debug).collect()
}

fn render(report: &Report, json: bool) -> io::Result<()> {
    let mut out = io::stdout().lock();
    if json {
        serde_json::to_writer(&mut out, report)?;
        writeln!(out)?;
    } else {
        writeln!(
            out,
            "{}: {} (exit {})",
            report.command, report.status, report.exit_code
        )?;
        for case in &report.cases {
            writeln!(out, "  {}: {}", escaped(&case.id), case.status)?;
            if let Some(m) = &case.message {
                writeln!(out, "    {}", escaped(m))?;
            }
            for change in &case.changes {
                writeln!(out, "    {} changed", change.channel)?;
                writeln!(out, "      expected: {}", change.expected)?;
                writeln!(out, "      actual:   {}", change.actual)?;
            }
        }
        if let Some(m) = &report.message {
            writeln!(out, "{}", escaped(m))?;
        }
        if let Some(hash) = &report.candidate_sha256 {
            writeln!(out, "candidate_sha256: {hash}")?;
        }
    }
    out.flush()
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().collect();
    let json = args.iter().any(|a| a == "--json");
    let cli = match Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        Err(e)
            if matches!(
                e.kind(),
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
            ) =>
        {
            return if e.print().is_ok() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(2)
            };
        }
        Err(e) => {
            let report = Report::new("cli").fail(Error::execution(e.to_string()));
            let _ = render(&report, json);
            return ExitCode::from(2);
        }
    };
    let mut report = Report::new(cli.action.name());
    if let Err(e) = run(&cli, &mut report) {
        report = report.fail(e);
    }
    match render(&report, cli.json) {
        Ok(()) => ExitCode::from(report.exit_code),
        Err(_) => {
            eprintln!("invar: could not write report");
            ExitCode::from(2)
        }
    }
}

#[test]
fn cli_definition_is_valid() {
    <Cli as clap::CommandFactory>::command().debug_assert();
}
