# RPC API

_Tracking this feature in [issue #8](https://github.com/sagan-software/eosio-rust/issues/8)_

All EOS apps need a way to talk to EOS nodes, to fetch table rows and to send transactions. In order for full-stack Rust-based EOS applications to come to fruition, there needs to be a solid RPC API. In Javascript there is `eosjs`, and something similar should exist for Rust.

Implementing this will be tricky since we need to support browser and server environments.

-   For browsers we need to support [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [`stdweb`](https://github.com/koute/stdweb/)
-   For servers we need to support [`hyper`](https://hyper.rs/guides/client/basic/)

This could get even more complicated if we decide to optionally support [futures](https://github.com/rust-lang-nursery/futures-rs). For the initial release futures will probably be mandatory.

There are a lot of things to consider so this may be a 1.0+ feature.
