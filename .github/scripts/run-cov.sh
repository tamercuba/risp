#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Ensure the target directory exists
mkdir -p target/tarpaulin

# Run cargo tarpaulin to generate coverage report in XML format
cargo tarpaulin --verbose --fail-under 40 --out Xml --output-dir target/tarpaulin

# Checking if coverage file exists 
ls -la target/tarpaulin