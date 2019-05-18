# Optional Dependencies

## wasm-gc

[wasm-gc](https://github.com/alexcrichton/wasm-gc) is a command-line tool that removes unused code in WASM files. It can be installed with Cargo:

```sh
cargo install wasm-gc
```

## Binaryen

[Binaryen](https://github.com/WebAssembly/binaryen) comes with a command-line tool called `wasm-opt` that optimizes WASM file sizes. Binaryen can be installed with most system package managers.

## WebAssembly Binary Toolkit (WABT)

[WABT](https://github.com/WebAssembly/wabt) comes with a command-line tool `wasm2wat` that can be used to create textual representations of WASM files, which can be useful for debugging. WABT can be installed with most system package managers.
