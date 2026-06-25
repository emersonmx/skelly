setup:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v lefthook &> /dev/null; then
        echo "lefthook not found, installing..."
        go install github.com/evilmartians/lefthook/v2@latest
    fi
    lefthook install

    if ! command -v bacon &> /dev/null; then
        echo "bacon not found, installing..."
        cargo install --locked bacon
    fi

@build *ARGS:
    cargo build {{ ARGS }}

@run *ARGS:
    cargo run {{ ARGS }}

@watch *ARGS:
    bacon {{ ARGS }}

@format:
    cargo fmt

@lint:
    cargo clippy --locked --workspace --all-targets --all-features


@lint-fix:
    cargo clippy --fix --allow-dirty --locked --workspace --all-targets --all-features

@ci:
    cargo fmt --check
    cargo clippy --locked --workspace --all-targets --profile ci --all-features

@test *ARGS:
    cargo test {{ ARGS }}

@clean *ARGS:
    cargo clean {{ ARGS }}
