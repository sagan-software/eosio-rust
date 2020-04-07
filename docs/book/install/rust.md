# Install Rust

EOSIO Rust works with stable Rust 1.31 and above.

Install Rust with `rustup` per the [official instructions](https://www.rust-lang.org/en-US/install.html):

```sh
curl https://sh.rustup.rs -sSf | sh
```

We will also need the `wasm32-unknown-unknown` target, which can be installed with `rustup`:

```sh
rustup target add wasm32-unknown-unknown
```
