# Advent of Code 2022

Advent of Code 2022 but it's in Rust with WASM as the compile target for some
reason.

## Getting Started

Install [Rust](https://www.rust-lang.org/) and [wasmtime](https://wasmtime.dev/).

```
rustup target add wasm32-wasi
```

Run program locally:

```
cat input.txt | cargo run
```

Build and run WASM:

```
cargo build --target wasm32-wasi --release
cat input.txt | wasmtime ./target/wasm32-wasi/release/<WASM FILE>
```