# glyphs-generator-rs

This repository is a Rust port of [glyphs-generator](https://github.com/v4nn4/glyphs-generator). it compiles to WebAssembly and is used in this [demo website](https://github.com/pages/v4nn4/glyphs-generator).

## Compile

To install dependencies and compile the Rust package, run

```bash
cargo build
```

In order to compile to WebAssembly, run 

```bash
wasm-pack build --target web
```

More information on https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm.

## Run tests

To run unit tests, run

```bash
cargo test
```

## Run examples

To run examples, run

```bash
cargo run --example hello
```