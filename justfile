#!/usr/bin/env just --justfile

alias b := build
alias l := lint
alias t := test

# Build release version of the binary
release:
  cargo build --release    

# Lint all rust code with clippy will automatically apply suggestions
lint:
  cargo clippy --fix

# Run all tests in project
test:
  cargo test

# Format with cargo
fmt:
  cargo fmt --

# Build normal dev binary
build:
   cargo build

# Setup/update pre-commit hooks (optional)
setup_precommit:
    pre-commit install --install-hooks