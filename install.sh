#!/bin/sh
set -eu

fail() {
    printf 'invar installer: %s\n' "$*" >&2
    exit 1
}

check_target() {
    if [ -e "$bin_dir/invar" ] || [ -L "$bin_dir/invar" ]; then
        existing=$(readlink "$bin_dir/invar") || fail "Refusing to overwrite $bin_dir/invar."
        case "$existing" in
            "$data_dir/"*/invar) ;;
            *) fail "Refusing to overwrite $bin_dir/invar. It belongs to another installation." ;;
        esac
    fi
}

main() {
    case "$(uname -s)/$(uname -m)" in
        Linux/x86_64) ;;
        *) fail 'The prebuilt release currently supports Linux/WSL x86_64 only.' ;;
    esac
    for tool in curl sha256sum tar mktemp mkdir rm mv ln cmp readlink grep; do
        command -v "$tool" >/dev/null 2>&1 || fail "Required command not found: $tool."
    done
    case "${HOME:-}" in
        /*) ;;
        *) fail 'HOME must be an absolute path.' ;;
    esac
    data_home=${XDG_DATA_HOME:-"$HOME/.local/share"}
    case "$data_home" in
        /*) ;;
        *) fail 'XDG_DATA_HOME must be an absolute path.' ;;
    esac
    version=0.1.0
    expected_sha256=f25515c65a53e5c3b1e41bf82314e4343bbdf54acfd31f80725094345f2161b1
    name="invar-v$version-x86_64-unknown-linux-musl"
    bin_dir="$HOME/.local/bin"
    data_dir="$data_home/invar"
    destination="$data_dir/$version"
    check_target
    mkdir -p "$data_dir"
    temp_dir=$(mktemp -d "$data_dir/.install.XXXXXX")
    trap 'rm -rf "$temp_dir"' 0
    trap 'exit 1' HUP INT TERM
    printf 'Downloading Invar %s...\n' "$version"
    curl --fail --silent --show-error --location --connect-timeout 15 --max-time 120 \
        --proto '=https' --proto-redir '=https' \
        "https://github.com/zhugez/invar/releases/download/v$version/$name.tar.gz" \
        --output "$temp_dir/$name.tar.gz"
    printf '%s  %s\n' "$expected_sha256" "$name.tar.gz" | (cd "$temp_dir" && sha256sum -c -)
    tar -xzf "$temp_dir/$name.tar.gz" -C "$temp_dir"
    [ "$("$temp_dir/$name/invar" --version)" = "invar $version" ] || fail 'The executable version does not match the release.'
    if [ -e "$destination" ] || [ -L "$destination" ]; then
        [ -d "$destination" ] && [ ! -L "$destination" ] && \
            cmp -s "$temp_dir/$name/invar" "$destination/invar" || \
            fail "Existing installation differs: $destination. Nothing was replaced."
    else
        mv "$temp_dir/$name" "$destination"
    fi
    mkdir -p "$bin_dir"
    check_target
    ln -s "$destination/invar" "$temp_dir/invar-link"
    mv -Tf "$temp_dir/invar-link" "$bin_dir/invar"
    printf 'Installed invar %s to %s/invar\n' "$version" "$bin_dir"
    case ":${PATH:-}:" in
        *":$bin_dir:"*) ;;
        *)
            shell=${SHELL:-sh}
            path_line='export PATH="$HOME/.local/bin:$PATH"'
            case "${shell##*/}" in
                bash) profile="$HOME/.bashrc" ;;
                zsh) mkdir -p "${ZDOTDIR:-$HOME}"; profile="${ZDOTDIR:-$HOME}/.zshrc" ;;
                sh|dash|ksh) profile="$HOME/.profile" ;;
                *) printf 'Add %s to your shell PATH.\n' "$bin_dir"; return ;;
            esac
            if [ ! -f "$profile" ] || ! grep -Fqx "$path_line" "$profile"; then
                printf '\n%s\n' "$path_line" >> "$profile"
            fi
            printf 'Open a new terminal to use invar.\n'
            ;;
    esac
}

main "$@"
