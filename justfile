#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias br := build-release
alias c := check
alias cc := code-coverage
alias ci := continues-integration
alias l := lint
alias t := test
alias r := run
alias rr := run-release

# list recipies
default:
    just --list

# build for debugging
build:
    cargo build --workspace

# build for release
build-release:
    cargo build --release --workspace

# check for compiler errors and warnings
check:
    cargo check --all-targets --workspace

# check for linter warnings and errors
lint:
    cargo clippy --all-targets --workspace

# run all tests
test:
    cargo test --workspace

# run code coverage (does not include doc-tests)
code-coverage:
    cargo +nightly llvm-cov clean --workspace
    cargo +nightly llvm-cov --branch --all-features --no-report --workspace
    cargo +nightly llvm-cov report --html --open --ignore-filename-regex "tests|test_dsl"

# clean all build output files
clean:
    cargo clean --workspace

# perform continues integration like tasks on the local machine
[env("RUSTFLAGS", "-D warnings")]
continues-integration:
    just lint
    just test

# run the application/game for debugging
run *args:
    cargo run --package lox-cli --bin lox {{ args }}

# run the application/game from release build
run-release *args:
    cargo run --release --package lox-cli --bin lox {{ args }}
