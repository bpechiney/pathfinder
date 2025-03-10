#!/usr/bin/env just --justfile

default:
  just --list

# Analyze the current package and report errors, but don't build object files.
[group: 'check']
check:
  cargo check

# Display outdated Rust dependencies.
[group: 'check']
outdated:
  cargo outdated -R

# Run the binary.
[group: 'misc']
run:
	cargo run

# Compile the current package.
[group: 'misc']
build:
  cargo build

# Remove artifacts that cargo has generated in the past.
[group: 'misc']
clean:
  cargo clean

# Execute tests matching `PATTERN`.
[group: 'test']
filter-test PATTERN:
  cargo test {{PATTERN}}

# Execute all unit tests.
[group: 'test']
unit-test:
  cargo test --lib --bin pathfinder

# Formats all bin and lib files of the current crate using rustfmt.
[group: 'quality']
fmt:
  cargo fmt --all

# Checks a package to catch common mistakes.
[group: 'quality']
lint:
  cargo clippy --all --all-targets -- --deny warnings
