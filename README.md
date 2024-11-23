# Description
- Rust based CLI tool to compare two input data sources and generate a data set of
unqiue rows in each data source and common rows in both data sources that have changed
or kept the same

# Dependencies
## Build from source
- Install and configure the [cargo package manager and rust toolchain](https://doc.rust-lang.org/cargo/getting-started/installation.html)


# Running and debugging
## Overview
a Makefile has been provided at your benefit to automatically run certain actions.
## Available options
- `test-release` => builds in release mode and runs cli binary
- `test-release-terminal` => builds in release mode and runs TUI flag through cli binary
- `test-debug` => builds in release mode and runs cli binary
- `test-debug-terminal` => builds in release mode and runs TUI flag through cli binary
- `build-debug` => builds in debug mode
- `build-release` => builds in debug mode
- `clean` => runs cargo clean and also deletes any current sqlite files created via `cargo run`
- `run-mysql` => start mysql docker container used during data generation conditions
