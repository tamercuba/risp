#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

mkdir -p target/coverage

# Run cargo test with code coverage flags
CARGO_INCREMENTAL=0 RUSTFLAGS='-C instrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test

# Generate the coverage report using grcov
# grcov . -s . --binary-path ./target/debug/ --branch --ignore-not-existing --ignore "/*" --ignore "target/debug/*" -o target/tarpaulin/coverage.xml
cargo llvm-cov --all-features --workspace --cobertura --ignore-filename-regex ^(?!.*\.rs$|test_.*\.rs$|mod\.rs$|lib\.rs$|LICENSE$|Makefile$).*$|^(test_.*\.rs|mod\.rs|lib\.rs|LICENSE|Makefile)$
 --output-path target/coverage/coverage.xml

ls -la target/coverage