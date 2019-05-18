# Install Rust

Install Rust with `rustup` per the [official instructions](https://www.rust-lang.org/en-US/install.html):

```sh
curl https://sh.rustup.rs -sSf | sh
```

This project requires nightly Rust and the `wasm32-unknown-unknown` target to be available, which can be installed with `rustup`:

```sh
rustup install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup default nightly
```
