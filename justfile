# Rust project checks

set positional-arguments

# List available commands
default:
    @just --list

# Run format, clippy-fix, and build in parallel
[parallel]
check: format clippy-fix build

# Format Rust files
format:
    cargo fmt --all

# Run clippy with all warnings
clippy:
    cargo clippy -- -W clippy::all

# Auto-fix clippy warnings
clippy-fix:
    cargo clippy --fix --allow-dirty -- -W clippy::all

# Build the project
build:
    cargo build --all

# Run the application
run *ARGS:
    cargo run -- "$@"
