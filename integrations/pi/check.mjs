import assert from "node:assert/strict";
import { copyFile, mkdtemp, mkdir, readFile, readdir, writeFile, rm, chmod, unlink, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { execFileSync } from "node:child_process";

const [sdkArgument, binaryArgument] = process.argv.slice(2);
if (!sdkArgument || !binaryArgument) throw new Error("Usage: node check.mjs /path/to/pi-coding-agent /path/to/invar");
const sdk = resolve(sdkArgument);
const publishedBinary = resolve(binaryArgument);
const source = join(dirname(fileURLToPath(import.meta.url)), "invar.ts");
const { loadExtensions } = await import(pathToFileURL(join(sdk, "dist/core/extensions/loader.js")).href);
const temporary = await mkdtemp(join(tmpdir(), "invar-pi-check-"));
const prior = { agent: process.env.PI_CODING_AGENT_DIR, binary: process.env.INVAR_BIN };
const checks = [];
const agentDirectory = join(temporary, "agent");
process.env.PI_CODING_AGENT_DIR = agentDirectory;
const project = join(temporary, "project with spaces");
const binary = join(temporary, "invar");
await mkdir(project);
await copyFile(publishedBinary, binary);
await chmod(binary, 0o700);
process.env.INVAR_BIN = binary;
const program = join(project, "report.mjs");
const original = 'import { appendFileSync } from "node:fs";\nappendFileSync("runs.log", "x");\nprocess.stdout.write("before\\n");\n';
await writeFile(program, original);
const packageFile = join(project, "package.json");
await writeFile(packageFile, JSON.stringify({ scripts: { test: "node report.mjs" } }));
await writeFile(join(project, "pnpm-lock.yaml"), "lockfileVersion: '9.0'\n");
await writeFile(join(project, "Cargo.toml"), '[package]\nname = "fixture"\nversion = "0.1.0"\n');
await writeFile(join(project, "Justfile"), "test:\n    echo test\n");
const runCount = async () => (await readFile(join(project, "runs.log"), "utf8").catch(error => { if (error.code !== "ENOENT") throw error; return ""; })).length;
const config = join(project, "invar.json");
const accepted = join(project, ".invar/accepted.json");
const candidate = join(project, ".invar/candidate.json");

async function harness(cwd = project) {
  const loaded = await loadExtensions([source], cwd);
  assert.deepEqual(loaded.errors, []);
  assert.equal(loaded.extensions.length, 1);
  const extension = loaded.extensions[0];
  const messages = [];
  const notices = [];
  const statuses = [];
  const selections = [];
  const configDrafts = [];
  loaded.runtime.sendMessage = message => messages.push(message);
  loaded.runtime.getAllTools = () => [{ name: "read", annotations: { readOnlyHint: true } }, { name: "edit" }];
  const options = { run: true, approve: true, trusted: true, candidateChange: false, editReview: false, cancelReview: false, commandChoice: "pnpm test (package.json)" };
  const ctx = {
    cwd, hasUI: true, mode: "rpc", waitForIdle: async () => {},
    isProjectTrusted: () => options.trusted,
    ui: {
      setStatus: (_key, value) => statuses.push(value),
      notify: (message, level) => notices.push({ message, level }),
      select: async (_title, choices) => {
        selections.push(choices);
        return choices.includes(options.commandChoice) ? options.commandChoice : choices.at(-1);
      },
      input: async () => JSON.stringify([process.execPath, "report.mjs"]),
      editor: async (title, text) => {
        if (title.startsWith("Review Invar configuration")) configDrafts.push(text);
        if (title.startsWith("Review captured")) {
          if (options.cancelReview) return undefined;
          if (options.editReview) return text + "changed";
        }
        return text;
      },
      confirm: async title => {
        if (title === "Save configuration and capture?") return options.run;
        if (options.candidateChange) await writeFile(join(cwd, ".invar/candidate.json"), "\n", { flag: "a" });
        return options.approve;
      },
    },
  };
  const emit = async (name, event = {}) => {
    let result;
    for (const handler of extension.handlers.get(name) ?? []) result = await handler({ type: name, ...event }, ctx);
    return result;
  };
  const command = async action => extension.commands.get("invar").handler(action, ctx);
  const dirty = () => emit("tool_execution_end", { toolName: "edit", isError: false });
  const boundary = (extra = {}) => emit("agent_before_settle", { entries: [], continue: false, outcome: "completed", context: {}, ...extra });
  return { options, ctx, emit, command, dirty, boundary, messages, notices, statuses, selections, configDrafts };
}

try {
  const h = await harness();
  h.options.run = false;
  await h.command("setup");
  assert.deepEqual(h.selections[0], ["pnpm test (package.json)", "cargo test (Cargo.toml)", "just test (Justfile)", "Enter a command manually"]);
  assert.match(h.configDrafts[0], /"argv": \[\s*"pnpm",\s*"test"\s*\]/);
  assert.equal(await runCount(), 0);
  await assert.rejects(readFile(config), { code: "ENOENT" });
  checks.push("pnpm lockfile, Cargo.toml, and Justfile offer commands before capture");
  checks.push("declining command capture runs nothing");

  h.options.commandChoice = "just test (Justfile)";
  await h.command("setup");
  assert.match(h.configDrafts.at(-1), /"argv": \[\s*"just",\s*"test"\s*\]/);

  await writeFile(packageFile, JSON.stringify({ packageManager: "bun@1.2.0", scripts: { test: "node report.mjs" } }));
  h.options.commandChoice = "bun run test (package.json)";
  await h.command("setup");
  assert.match(h.configDrafts.at(-1), /"argv": \[\s*"bun",\s*"run",\s*"test"\s*\]/);

  await writeFile(packageFile, JSON.stringify({ packageManager: "yarn@4.0.0", scripts: { test: "node report.mjs" } }));
  h.options.commandChoice = "yarn test (package.json)";
  await h.command("setup");
  assert.match(h.configDrafts.at(-1), /"argv": \[\s*"yarn",\s*"test"\s*\]/);

  await unlink(join(project, "pnpm-lock.yaml"));
  await writeFile(packageFile, JSON.stringify({ scripts: { test: "node report.mjs" } }));
  h.options.commandChoice = "npm test (package.json)";
  await h.command("setup");
  assert.match(h.configDrafts.at(-1), /"argv": \[\s*"npm",\s*"test"\s*\]/);

  h.options.commandChoice = "cargo test (Cargo.toml)";
  await h.command("setup");
  assert.match(h.configDrafts.at(-1), /"argv": \[\s*"cargo",\s*"test"\s*\]/);
  h.options.commandChoice = "Enter a command manually";
  h.options.run = true;
  h.options.approve = false;
  await h.command("setup");
  assert.equal(await runCount(), 1);
  await assert.rejects(readFile(accepted), { code: "ENOENT" });
  assert.ok((await readFile(candidate)).length);
  await h.dirty();
  assert.equal(await h.boundary(), undefined);
  assert.equal(await runCount(), 1);
  checks.push("capture does not approve or enable automatic execution");

  h.options.approve = true;
  await h.command("setup");
  assert.equal(await runCount(), 3);
  assert.equal(h.notices.filter(notice => notice.level === "error").length, 0, JSON.stringify(h.notices));
  assert.ok(h.messages.at(-1).content.includes("Initial verification passed"));
  const baseline = await readFile(accepted);
  const configBytes = await readFile(config);
  const candidateBytes = await readFile(candidate);
  const savedName = (await readdir(join(agentDirectory, "invar"))).find(name => name.endsWith(".json"));
  const savedFile = join(agentDirectory, "invar", savedName);
  const registration = await readFile(savedFile);
  checks.push("explicit approval enables a verified project");

  const beforeRead = await runCount();
  await h.emit("tool_execution_end", { toolName: "read" });
  assert.equal(await h.boundary(), undefined);
  assert.equal(await runCount(), beforeRead);
  checks.push("read-only tools do not trigger verification");

  await h.dirty();
  const other = { type: "custom", customType: "other-extension", data: { value: 1 } };
  const pass = await h.boundary({ entries: [other] });
  assert.equal(pass.continue, undefined);
  assert.deepEqual(pass.entries[0], other);
  assert.ok(pass.entries.at(-1).content.startsWith("Invar PASS"));
  assert.deepEqual(await readFile(accepted), baseline);
  assert.deepEqual(await readFile(candidate), candidateBytes);
  assert.deepEqual(await readFile(config), configBytes);
  checks.push("final-boundary verification preserves project artifacts and other proposals");

  await writeFile(program, original.replace('"before\\n"', '"after\\n"'));
  await h.dirty();
  const regression = await h.boundary();
  assert.equal(regression.continue, true);
  assert.ok(regression.entries[0].content.includes('"status":"regression"'));
  assert.deepEqual(await readFile(accepted), baseline);
  await writeFile(program, original);
  await h.dirty();
  const repaired = await h.boundary();
  assert.ok(repaired.entries[0].content.startsWith("Invar PASS"));
  assert.equal(repaired.continue, undefined);
  checks.push("real regression requests repair and restored behavior passes");

  await h.emit("before_agent_start", { messageType: "user" });
  await writeFile(program, original.replace('"before\\n"', '"after\\n"'));
  await h.dirty();
  assert.equal((await h.boundary()).continue, true);
  const unresolved = await h.boundary();
  assert.equal(unresolved.continue, undefined);
  assert.ok(unresolved.entries[0].content.includes("Do not claim completion"));
  await writeFile(program, original);
  await h.dirty();
  await h.boundary();
  checks.push("automatic repair requests are bounded");

  const beforePinCheck = await runCount();
  await writeFile(config, Buffer.concat([configBytes, Buffer.from("\n")]));
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.includes("Reviewed inputs changed: config"));
  assert.equal(await runCount(), beforePinCheck);
  await writeFile(config, configBytes);
  checks.push("configuration changes stop execution");

  await writeFile(program, original.replace('"before\\n"', '"after\\n"'));
  execFileSync(binary, ["--project", project, "--json", "capture"]);
  await copyFile(candidate, accepted);
  assert.equal(JSON.parse(execFileSync(binary, ["--project", project, "--json", "verify"])).status, "pass");
  const beforeForged = await runCount();
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.includes("Reviewed inputs changed: baseline"));
  assert.equal(await runCount(), beforeForged);
  await writeFile(accepted, baseline);
  await writeFile(program, original);
  checks.push("a locally rewritten baseline cannot produce an adapter pass");

  const executable = await readFile(binary);
  await writeFile(binary, Buffer.concat([executable, Buffer.from("\0")]));
  const beforeBinary = await runCount();
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.includes("executable"));
  assert.equal(await runCount(), beforeBinary);
  await writeFile(binary, executable);
  checks.push("executable changes stop execution");

  await unlink(config);
  const alternate = join(project, "alternate.json");
  await writeFile(alternate, configBytes);
  await symlink(alternate, config);
  const beforeSpecial = await runCount();
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.startsWith("Invar adapter ERROR"));
  assert.equal(await runCount(), beforeSpecial);
  await unlink(config);
  execFileSync("mkfifo", [config]);
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.includes("Invalid or oversized file"));
  assert.equal(await runCount(), beforeSpecial);
  await unlink(config);
  await writeFile(config, configBytes);
  checks.push("symlinks and FIFOs fail closed without running commands");

  h.options.trusted = false;
  const beforeTrust = await runCount();
  await h.dirty();
  assert.ok((await h.boundary()).entries[0].content.includes("not trusted"));
  assert.equal(await runCount(), beforeTrust);
  h.options.trusted = true;
  await h.dirty();
  const beforePending = await runCount();
  assert.equal(await h.boundary({ continue: true }), undefined);
  assert.equal(await h.boundary({ outcome: "aborted" }), undefined);
  assert.equal(await runCount(), beforePending);
  checks.push("untrusted projects, aborts, and pending continuations do not execute commands");

  const restored = await harness();
  await restored.emit("session_start");
  assert.equal(restored.statuses.at(-1), "invar: armed");
  await restored.dirty();
  assert.ok((await restored.boundary()).entries[0].content.startsWith("Invar PASS"));
  checks.push("registration survives extension reload");

  restored.options.candidateChange = true;
  await restored.command("setup");
  assert.ok(restored.notices.some(notice => notice.level === "error"));
  assert.deepEqual(await readFile(accepted), baseline);
  assert.deepEqual(await readFile(savedFile), registration);
  checks.push("candidate changes during review cannot be approved");

  restored.options.candidateChange = false;
  restored.options.editReview = true;
  await restored.command("setup");
  assert.deepEqual(await readFile(accepted), baseline);
  assert.deepEqual(await readFile(savedFile), registration);
  checks.push("editing the review preview never changes or approves a candidate");

  await restored.command("off");
  assert.deepEqual(await readFile(accepted), baseline);
  await restored.dirty();
  const beforeOff = await runCount();
  assert.equal(await restored.boundary(), undefined);
  assert.equal(await runCount(), beforeOff);
  await assert.rejects(readFile(savedFile), { code: "ENOENT" });
  checks.push("off preserves baselines and stops automatic execution");

  console.log(JSON.stringify({ passed: checks.length, checks }, null, 2));
} finally {
  if (prior.agent === undefined) delete process.env.PI_CODING_AGENT_DIR;
  else process.env.PI_CODING_AGENT_DIR = prior.agent;
  if (prior.binary === undefined) delete process.env.INVAR_BIN;
  else process.env.INVAR_BIN = prior.binary;
  await rm(temporary, { recursive: true, force: true });
}
