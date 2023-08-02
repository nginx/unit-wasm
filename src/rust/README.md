# Quickstart in Developing Rust WebAssembly Modules for Unit

The current version is published to crates.io. To get started with the SDK
include `unit-wasm` as dependency.

```
cargo add unit-wasm
```

## Prerequisites

- target add wasm32-wasi. `rustup target add wasm32-wasi`

## From Source

The Rust implementation is in an early stage. If you would like to build the
crate by yourself, we have to generate the `libunit-wasm` first. This step is
NOT included in the build process.
