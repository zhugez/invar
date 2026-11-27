fmt:
    cargo fmt --check

lint:
    cargo clippy --locked --all-targets -- -D warnings

test:
    cargo test --locked --all-targets

msrv:
    cargo +1.88.0 test --locked --all-targets

check: fmt lint test msrv
