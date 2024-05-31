#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Run cargo tarpaulin to generate coverage report in XML format
cargo tarpaulin --out Xml --output-dir target/tarpaulin || true

# Ensure the target directory exists
mkdir -p target/tarpaulin

# Move the generated coverage report to the target directory
mv tarpaulin-report.xml target/tarpaulin/coverage.xml