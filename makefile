################################################################################
# Author: Kenneth Alexander Jenkins <biz@alexj.io>
# Date: 28th of March, 2025
# Source: https://github.com/rockenman1234
# Copyright: GPLv2 License (see LICENSE)
# Description: GNU Makefile for `cached`
################################################################################
# Makefile for the `cached` project
build:
	@echo "Building the program..."
	cargo build --release --verbose

run:
	@echo "Running the program..."
	cargo run --release

clean:
	@echo "Cleaning up..."
	cargo clean

all: clean build run

################################################################################
# Debug build
# This target builds the program in debug mode
debug:
	@echo "Building the program in debug mode..."
	cargo build --verbose