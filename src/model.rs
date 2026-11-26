use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Component, Path, PathBuf};

pub const CONFIG_LIMIT: u64 = 256 * 1024;
pub const SNAPSHOT_LIMIT: u64 = 20 * 1024 * 1024;
pub const TOTAL_OUTPUT_LIMIT: usize = 8 * 1024 * 1024;
pub const STREAM_LIMIT: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Error {
    pub code: u8,
    pub message: String,
}

impl Error {
    pub fn execution(message: impl Into<String>) -> Self {
        Self {
            code: 2,
            message: message.into(),
        }
    }
    pub fn baseline(message: impl Into<String>) -> Self {
        Self {
            code: 3,
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub cases: Vec<Case>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub argv: Vec<String>,
    #[serde(default = "default_cwd")]
    pub cwd: String,
    #[serde(default, deserialize_with = "unique_env")]
    pub env: BTreeMap<String, String>,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default = "default_output")]
    pub max_output_bytes: usize,
}

fn default_cwd() -> String {
    ".".into()
}
fn default_timeout() -> u64 {
    1000
}
fn default_output() -> usize {
    STREAM_LIMIT
}

fn unique_env<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<BTreeMap<String, String>, D::Error> {
    struct Unique;
    impl<'de> serde::de::Visitor<'de> for Unique {
        type Value = BTreeMap<String, String>;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("an environment object with unique keys")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> std::result::Result<Self::Value, M::Error> {
            let mut values = BTreeMap::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                if values.insert(key, value).is_some() {
                    return Err(serde::de::Error::custom("duplicate environment key"));
                }
            }
            Ok(values)
        }
    }
    deserializer.deserialize_map(Unique)
}

impl Config {
    pub fn sample() -> Self {
        Self {
            version: 1,
            cases: vec![Case {
                id: "hello".into(),
                argv: vec!["printf".into(), "hello\n".into()],
                cwd: default_cwd(),
                env: BTreeMap::new(),
                timeout_ms: default_timeout(),
                max_output_bytes: default_output(),
            }],
        }
    }

    pub fn validate(&self) -> std::result::Result<(), String> {
        if self.version != 1 {
            return Err("unsupported configuration version".into());
        }
        if self.cases.is_empty() || self.cases.len() > 32 {
            return Err("configuration requires 1 through 32 cases".into());
        }
        let mut ids = BTreeSet::new();
        for c in &self.cases {
            if c.id.is_empty()
                || c.id.len() > 64
                || !c
                    .id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            {
                return Err(
                    "case IDs require 1–64 ASCII letters, digits, underscores, or hyphens".into(),
                );
            }
            if !ids.insert(&c.id) {
                return Err(format!("duplicate case ID: {}", c.id));
            }
            if c.argv.is_empty() || c.argv[0].is_empty() || c.argv.iter().any(|a| a.contains('\0'))
            {
                return Err(format!(
                    "{}: argv requires a program and NUL-free arguments",
                    c.id
                ));
            }
            let cwd = Path::new(&c.cwd);
            if c.cwd.is_empty()
                || c.cwd.contains('\0')
                || cwd.is_absolute()
                || cwd
                    .components()
                    .any(|p| !matches!(p, Component::Normal(_) | Component::CurDir))
            {
                return Err(format!("{}: cwd must remain within the project", c.id));
            }
            if !(1..=30000).contains(&c.timeout_ms)
                || !(1..=STREAM_LIMIT).contains(&c.max_output_bytes)
            {
                return Err(format!(
                    "{}: timeout or output limit outside supported bounds",
                    c.id
                ));
            }
            if c.env
                .iter()
                .any(|(k, v)| k.is_empty() || k.contains(['=', '\0']) || v.contains('\0'))
            {
                return Err(format!("{}: invalid environment key or value", c.id));
            }
        }
        Ok(())
    }

    pub fn fingerprint(&self) -> String {
        digest(&serde_json::to_vec(self).expect("serializable configuration"))
    }
}

pub fn working_dir(root: &Path, case: &Case) -> Result<PathBuf> {
    let dir = root
        .join(&case.cwd)
        .canonicalize()
        .map_err(|e| Error::execution(format!("{}: cwd: {e}", case.id)))?;
    if !dir.starts_with(root) || !dir.is_dir() {
        return Err(Error::execution(format!(
            "{}: cwd must resolve to a directory within the project",
            case.id
        )));
    }
    Ok(dir)
}

pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut out, "{b:02x}").expect("writing to String");
    }
    out
}

pub fn digest(bytes: &[u8]) -> String {
    encode(&Sha256::digest(bytes))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Blob {
    pub hex: String,
    pub sha256: String,
}

impl Blob {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            hex: encode(bytes),
            sha256: digest(bytes),
        }
    }
    pub fn bytes(&self, limit: usize) -> std::result::Result<Vec<u8>, String> {
        if self.hex.len() > limit * 2 || !self.hex.len().is_multiple_of(2) {
            return Err("invalid or excessive output artifact length".into());
        }
        fn digit(b: u8) -> Option<u8> {
            match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                _ => None,
            }
        }
        let bytes: Option<Vec<u8>> = self
            .hex
            .as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| Some(digit(p[0])? * 16 + digit(p[1])?))
            .collect();
        let bytes = bytes.ok_or("output artifact requires lowercase hexadecimal")?;
        if digest(&bytes) != self.sha256 {
            return Err("output artifact SHA-256 mismatch".into());
        }
        Ok(bytes)
    }
    pub fn len(&self) -> usize {
        self.hex.len() / 2
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub id: String,
    pub exit_code: i32,
    pub stdout: Blob,
    pub stderr: Blob,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub version: u32,
    pub config: Config,
    pub config_sha256: String,
    pub observations: Vec<Observation>,
}

impl Snapshot {
    pub fn new(config: Config, observations: Vec<Observation>) -> Self {
        Self {
            version: 1,
            config_sha256: config.fingerprint(),
            config,
            observations,
        }
    }
    pub fn validate(&self, config: &Config) -> Result<()> {
        let invalid = |m| Error::baseline(format!("invalid baseline: {m}"));
        self.config.validate().map_err(&invalid)?;
        if self.version != 1 || self.config_sha256 != self.config.fingerprint() {
            return Err(invalid("version or configuration digest mismatch".into()));
        }
        if &self.config != config {
            return Err(invalid(
                "configuration drift: capture and review a new candidate".into(),
            ));
        }
        if self.observations.len() != config.cases.len() {
            return Err(invalid("case count mismatch".into()));
        }
        let mut total = 0;
        for (o, c) in self.observations.iter().zip(&config.cases) {
            if o.id != c.id || !(0..=255).contains(&o.exit_code) {
                return Err(invalid("case identity/order or exit code mismatch".into()));
            }
            for b in [&o.stdout, &o.stderr] {
                total += b.bytes(c.max_output_bytes).map_err(&invalid)?.len();
                if total > TOTAL_OUTPUT_LIMIT {
                    return Err(invalid("aggregate output exceeds 8 MiB".into()));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct Change {
    pub channel: String,
    pub expected: String,
    pub actual: String,
}

#[derive(Debug, Serialize)]
pub struct CaseResult {
    pub id: String,
    pub status: String,
    pub message: Option<String>,
    pub changes: Vec<Change>,
}

impl CaseResult {
    pub fn new(id: &str, status: &str) -> Self {
        Self {
            id: id.into(),
            status: status.into(),
            message: None,
            changes: Vec::new(),
        }
    }
    pub fn error(id: &str, message: String) -> Self {
        Self {
            message: Some(message),
            ..Self::new(id, "error")
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub version: u32,
    pub command: String,
    pub status: String,
    pub exit_code: u8,
    pub message: Option<String>,
    pub candidate_sha256: Option<String>,
    pub cases: Vec<CaseResult>,
}

impl Report {
    pub fn new(command: &str) -> Self {
        Self {
            version: 1,
            command: command.into(),
            status: "ok".into(),
            exit_code: 0,
            message: None,
            candidate_sha256: None,
            cases: Vec::new(),
        }
    }
    pub fn fail(mut self, error: Error) -> Self {
        self.exit_code = error.code;
        self.status = if error.code == 3 {
            "invalid_baseline"
        } else {
            "error"
        }
        .into();
        self.message = Some(error.message);
        self
    }
}

pub fn compare(expected: &Observation, actual: &Observation) -> CaseResult {
    let mut result = CaseResult::new(&actual.id, "pass");
    if expected.exit_code != actual.exit_code {
        result.changes.push(Change {
            channel: "exit_code".into(),
            expected: expected.exit_code.to_string(),
            actual: actual.exit_code.to_string(),
        });
    }
    for (channel, e, a) in [
        ("stdout", &expected.stdout, &actual.stdout),
        ("stderr", &expected.stderr, &actual.stderr),
    ] {
        if e.hex == a.hex {
            continue;
        }
        let eb = e.bytes(STREAM_LIMIT).expect("valid expected bytes");
        let ab = a.bytes(STREAM_LIMIT).expect("valid actual bytes");
        let offset = eb
            .iter()
            .zip(&ab)
            .position(|(e, a)| e != a)
            .unwrap_or(eb.len().min(ab.len()));
        let summary = |b: &[u8], hash: &str| {
            let preview = &b[offset.min(b.len())..(offset + 48).min(b.len())];
            let escaped: String = preview
                .iter()
                .flat_map(|b| std::ascii::escape_default(*b))
                .map(char::from)
                .collect();
            format!(
                "{} bytes, sha256={hash}, offset={offset}, hex={}, escaped=\"{escaped}\"",
                b.len(),
                encode(preview)
            )
        };
        result.changes.push(Change {
            channel: channel.into(),
            expected: summary(&eb, &e.sha256),
            actual: summary(&ab, &a.sha256),
        });
    }
    if !result.changes.is_empty() {
        result.status = "regression".into();
    }
    result
}
