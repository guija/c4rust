# c4rust

Model your c4 architecture using the programming language Rust

[![Rust](https://github.com/guija/c4rust/actions/workflows/rust.yml/badge.svg)](https://github.com/guija/c4rust/actions/workflows/rust.yml)

## Build and test

- Compile: `cargo b`
- Run tests: `cargo t`
- Run main: `cargo r`
- Run example: `cargo r --example <folder name in /examples>`

## Example usage

- `cargo r --example company | dot -Tpng > container.png && eog container.png`
- `cargo r --example company | dot -Tsvg > container.svg && firefox container.svg`