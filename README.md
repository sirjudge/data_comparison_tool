# Description
- Rust based CLI tool to compare two input data sources and generate a data set of
unqiue rows in each data source and common rows in both data sources that have changed
or kept the same

# Dependencies
## Build from source
- Install and configure the [cargo package manager and rust toolchain](https://doc.rust-lang.org/cargo/getting-started/installation.html)


# Running and debugging and compiling
### Compilation
#### configure rustup
If not already installed, use the following link to install and
configure `rustup`, a tool for managing the rust tool chain
- [https://rustup.rs/](https://rustup.rs/)

Once installed, set the default channel channel
```shell
# stable
rustup override set stable
rustup update stable

# nightly
rustup override set nightly
rustup update nightly
```

#### Build using cargo
```shell
# creates debug build
cargo build

# creates release build
cargo build --release
```
### Makefile
A `Makefile` has been provided to speed up the dev cycle and help make incremental testing
via `cargo test` and running the binary with CLI flags
#### testing commands
- `test-release` => builds in release mode and runs cli binary
- `test-release-terminal` => builds in release mode and runs TUI flag through cli binary
- `test-debug` => builds in release mode and runs cli binary
- `test-debug-terminal` => builds in release mode and runs TUI flag through cli binary
#### Build commands
- `build-debug` => builds in debug mode
- `build-release` => builds in debug mode
#### Clean commands
- `clean` => runs cargo clean and also deletes any current sqlite files created via `cargo run`
