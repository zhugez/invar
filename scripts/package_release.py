import hashlib
import json
from pathlib import Path
import platform
import shutil
import subprocess
import tarfile
import tempfile


ROOT = Path(__file__).resolve().parents[1]
TARGET = "x86_64-unknown-linux-musl"


def output(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True, timeout=180).strip()


def main():
    if (platform.system(), platform.machine()) != ("Linux", "x86_64"):
        raise SystemExit("Release packaging currently requires Linux x86_64.")
    subprocess.run(["git", "diff", "--exit-code", "HEAD", "--", "Cargo.toml", "Cargo.lock", "src"], cwd=ROOT, check=True)
    metadata = json.loads(output("cargo", "metadata", "--locked", "--offline", "--format-version", "1", "--filter-platform", TARGET))
    packages = {p["id"]: p for p in metadata["packages"]}
    package = packages[metadata["resolve"]["root"]]
    version = package["version"]
    name = f"invar-v{version}-{TARGET}"
    target_dir = Path(metadata["target_directory"])
    dist = target_dir / "dist"
    dist.mkdir(parents=True, exist_ok=True)
    subprocess.run(["cargo", "build", "--locked", "--offline", "--release", "--target", TARGET], cwd=ROOT, check=True, timeout=180)
    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
    selected, pending = set(), [package["id"]]
    while pending:
        current = pending.pop()
        if current not in selected:
            selected.add(current)
            pending.extend(d["pkg"] for d in nodes[current]["deps"])
    with tempfile.TemporaryDirectory(prefix="invar-release-") as directory:
        stage = Path(directory) / name
        stage.mkdir()
        binary = stage / "invar"
        shutil.copyfile(target_dir / TARGET / "release/invar", binary)
        binary.chmod(0o755)
        subprocess.run(["strip", str(binary)], check=True)
        assert output(str(binary), "--version") == f"invar {version}"
        assert "INTERP" not in output("readelf", "--program-headers", str(binary))
        assert "(NEEDED)" not in output("readelf", "--dynamic", str(binary))
        for filename in ("LICENSE", "Cargo.lock"):
            shutil.copyfile(ROOT / filename, stage / filename)
        shutil.copyfile(ROOT / "docs/releases" / f"v{version}.md", stage / "README.md")
        notices = ["Invar binary distribution: third-party notices", "", "This inventory includes build-time dependencies. See each retained license.", ""]
        for identifier in sorted(selected - {package["id"]}):
            dependency = packages[identifier]
            source = Path(dependency["manifest_path"]).parent
            files = sorted(p for p in source.iterdir() if p.is_file() and p.name.lower().startswith(("license", "copying", "notice")))
            if not files:
                raise RuntimeError(f"No license files found for {dependency['name']}.")
            destination = stage / "licenses" / f"{dependency['name']}-{dependency['version']}"
            destination.mkdir(parents=True)
            for file in files:
                shutil.copyfile(file, destination / file.name)
            notices.append(f"{dependency['name']} {dependency['version']}: {dependency['license']} ({dependency['source']})")
        rust_docs = Path(output("rustc", "--print", "sysroot")) / "share/doc/rust"
        rust_notices = stage / "licenses/rust"
        rust_notices.mkdir()
        shutil.copyfile(rust_docs / "COPYRIGHT-library.html", rust_notices / "COPYRIGHT-library.html")
        shutil.copytree(rust_docs / "licenses", rust_notices / "licenses")
        musl_notices = stage / "licenses/musl"
        musl_notices.mkdir()
        shutil.copyfile(ROOT / "docs/licenses/musl-COPYRIGHT", musl_notices / "COPYRIGHT")
        notices.extend(["", "Rust standard-library notices: licenses/rust/COPYRIGHT-library.html", "musl notices: licenses/musl/COPYRIGHT", "musl notice provenance: https://git.musl-libc.org/cgit/musl/tree/COPYRIGHT?id=7813e9119eb5fe681b6750b8a033a0490bca0941", ""])
        (stage / "THIRD-PARTY-NOTICES.txt").write_text("\n".join(notices), encoding="utf-8")
        provenance = {"version": version, "target": TARGET, "source_commit": output("git", "rev-parse", "HEAD"), "rustc": output("rustc", "--version"), "cargo_lock_sha256": hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest(), "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest()}
        (stage / "BUILD-INFO.json").write_text(json.dumps(provenance, indent=2) + "\n", encoding="utf-8")
        archive = dist / f"{name}.tar.gz"
        with tarfile.open(archive, "w:gz") as bundle:
            bundle.add(stage, arcname=name)
        with tarfile.open(archive) as bundle:
            members = bundle.getmembers()
            assert all(m.name == name or m.name.startswith(name + "/") for m in members)
            assert all(m.isfile() or m.isdir() for m in members)
            archived_binary = bundle.extractfile(f"{name}/invar")
            assert archived_binary is not None
            assert hashlib.sha256(archived_binary.read()).hexdigest() == provenance["binary_sha256"]
            assert bundle.getmember(f"{name}/invar").mode & 0o111
            assert bundle.getmember(f"{name}/licenses/unicode-ident-1.0.27/LICENSE-UNICODE").isfile()
            assert bundle.getmember(f"{name}/licenses/musl/COPYRIGHT").isfile()
        digest = hashlib.sha256(archive.read_bytes()).hexdigest()
        (dist / "SHA256SUMS").write_text(f"{digest}  {archive.name}\n", encoding="ascii")
        print(f"PASS: {archive}; {len(selected) - 1} dependency notice sets; static binary; archive checks.")
        print(f"SHA256: {digest}")


if __name__ == "__main__":
    main()
