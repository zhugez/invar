import argparse
import hashlib
import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]
SCRIPT = (ROOT / "install.sh").read_text()
PATH_LINE = 'export PATH="$HOME/.local/bin:$PATH"'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive", type=Path, required=True)
    archive = parser.parse_args().archive.resolve(strict=True)
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    assert f"expected_sha256={digest}\n" in SCRIPT, "Use the published release archive."
    subprocess.run(["sh", "-n", str(ROOT / "install.sh")], check=True)
    checked = []
    with tempfile.TemporaryDirectory(prefix="invar-installer-check-") as directory:
        workspace = Path(directory)
        tools = workspace / "tools"
        tools.mkdir()
        (tools / "curl").write_text('''#!/bin/sh
set -eu
[ "${INVAR_TEST_DOWNLOAD_FAIL:-0}" = 0 ] || exit 22
while [ "$#" -gt 0 ]; do
    if [ "$1" = --output ]; then
        shift
        cp "$INVAR_TEST_ARCHIVE" "$1"
        exit
    fi
    shift
done
exit 2
''')
        (tools / "uname").write_text('''#!/bin/sh
case "$1" in
    -s) printf '%s\\n' "${INVAR_TEST_OS:-Linux}" ;;
    -m) printf '%s\\n' "${INVAR_TEST_ARCH:-x86_64}" ;;
esac
''')
        for tool in tools.iterdir():
            tool.chmod(0o755)

        def case(name, **overrides):
            home = workspace / name / "home with spaces"
            home.mkdir(parents=True)
            environment = {"HOME": str(home), "PATH": f"{tools}:/usr/bin:/bin", "SHELL": "/bin/bash", "INVAR_TEST_ARCHIVE": str(archive)}
            environment.update(overrides)
            return home, environment

        def invoke(home, environment, source=SCRIPT, success=True):
            result = subprocess.run(["/bin/sh"], input=source, text=True, capture_output=True, env=environment, cwd=home, timeout=20)
            assert (result.returncode == 0) == success, (result.returncode, result.stdout, result.stderr)
            data_dir = Path(environment.get("XDG_DATA_HOME", home / ".local/share")) / "invar"
            assert not list(data_dir.glob(".install.*")), "Temporary files were not removed."
            return result

        home, environment = case("install-and-repeat")
        profile = home / ".bashrc"
        original_profile = "INVAR_USER_SETTING=preserved\n"
        profile.write_text(original_profile)
        invoke(home, environment)
        binary = home / ".local/bin/invar"
        package = home / ".local/share/invar/0.1.0"
        assert binary.is_symlink() and binary.resolve() == package / "invar"
        assert os.access(binary, os.X_OK)
        for notice in ("LICENSE", "BUILD-INFO.json", "THIRD-PARTY-NOTICES.txt", "licenses/musl/COPYRIGHT", "licenses/unicode-ident-1.0.27/LICENSE-UNICODE"):
            assert (package / notice).is_file(), notice
        installed_bytes = binary.read_bytes()
        invoke(home, environment)
        assert profile.read_text().startswith(original_profile)
        assert profile.read_text().splitlines().count(PATH_LINE) == 1
        assert binary.read_bytes() == installed_bytes
        version = subprocess.check_output(["/bin/sh", "-c", '. "$HOME/.bashrc"; invar --version'], env=environment, text=True).strip()
        assert version == "invar 0.1.0"
        checked.append("install, notices, spaces in HOME, PATH, and idempotent reinstall")

        for name, overrides in (("macos", {"INVAR_TEST_OS": "Darwin"}), ("arm64", {"INVAR_TEST_ARCH": "aarch64"})):
            home, environment = case(name, **overrides)
            result = invoke(home, environment, success=False)
            assert "Linux/WSL x86_64 only" in result.stderr and "Downloading" not in result.stdout
            assert not (home / ".local").exists()
            checked.append(name)

        for name in ("existing-file", "foreign-symlink"):
            home, environment = case(name)
            binary = home / ".local/bin/invar"
            binary.parent.mkdir(parents=True)
            original = home / "existing-tool"
            original.write_bytes(b"preserve this tool\n")
            if name == "foreign-symlink":
                binary.symlink_to(original)
            else:
                binary.write_bytes(original.read_bytes())
            result = invoke(home, environment, success=False)
            assert "Refusing to overwrite" in result.stderr
            assert binary.read_bytes() == original.read_bytes()
            checked.append(name)

        corrupt = workspace / "corrupt.tar.gz"
        corrupt.write_bytes(archive.read_bytes() + b"corruption")
        for name, overrides in (("checksum-failure", {"INVAR_TEST_ARCHIVE": str(corrupt)}), ("download-failure", {"INVAR_TEST_DOWNLOAD_FAIL": "1"}), ("managed-upgrade", {})):
            home, environment = case(name, **overrides)
            old = home / ".local/share/invar/0.0.0/invar"
            old.parent.mkdir(parents=True)
            old.write_bytes(b"previous installation\n")
            binary = home / ".local/bin/invar"
            binary.parent.mkdir(parents=True)
            binary.symlink_to(old)
            invoke(home, environment, success=name == "managed-upgrade")
            assert old.read_bytes() == b"previous installation\n"
            assert binary.resolve() == (home / ".local/share/invar/0.1.0/invar" if name == "managed-upgrade" else old)
            if name != "managed-upgrade":
                assert not (home / ".bashrc").exists()
            checked.append(name)

        home, environment = case("same-version-conflict")
        binary = home / ".local/bin/invar"
        changed = home / ".local/share/invar/0.1.0/invar"
        changed.parent.mkdir(parents=True)
        changed.write_bytes(b"locally modified installation\n")
        binary.parent.mkdir(parents=True)
        binary.symlink_to(changed)
        result = invoke(home, environment, success=False)
        assert "Existing installation differs" in result.stderr
        assert binary.resolve() == changed and changed.read_bytes() == b"locally modified installation\n"
        checked.append("different same-version installation is preserved")

        home, environment = case("existing-path")
        data_home = workspace / "custom data directory"
        environment.update(XDG_DATA_HOME=str(data_home), SHELL="/bin/fish")
        environment["PATH"] = str(home / ".local/bin") + ":" + environment["PATH"]
        invoke(home, environment)
        assert (home / ".local/bin/invar").resolve() == data_home / "invar/0.1.0/invar"
        assert not (home / ".profile").exists() and not (home / ".bashrc").exists()
        assert subprocess.check_output(["invar", "--version"], env=environment, text=True).strip() == "invar 0.1.0"
        checked.append("existing PATH and XDG data directory")

        home, environment = case("zsh")
        zdotdir = home / "custom zsh directory"
        environment.update(SHELL="/bin/zsh", ZDOTDIR=str(zdotdir))
        invoke(home, environment)
        assert PATH_LINE in (zdotdir / ".zshrc").read_text().splitlines()
        assert not (home / ".bashrc").exists()
        checked.append("zsh configuration path")

        home, environment = case("truncated-installer")
        invoke(home, environment, source=SCRIPT.rsplit("\n}", 1)[0], success=False)
        assert not (home / ".local").exists()
        checked.append("truncated installer executes no installation")
    print(f"PASS: {len(checked)} installer checks. Temporary homes removed.")
    for check in checked:
        print(f"  {check}")


if __name__ == "__main__":
    main()
