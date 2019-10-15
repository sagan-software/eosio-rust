# CLI

_Tracking this feature in [issue #9](https://github.com/sagan-software/eosio-rust/issues/9)_

We already have several features that need CLIs. Consolidating all our CLIs under one CLI will make things simpler for developers and allow us to add new commands later on.

Commands should be implemented to:

-   Create a new `eosio-rust` project, e.g. `eosio-rust new`
-   Generate an ABI file, e.g. `eosio-rust to-abi`
-   Generate Rust from an ABI, e.g. `eosio-rust from-abi`
-   Manage table schemas, e.g. `eosio-rust schema`
-   Run unit tests, e.g. `eosio-rust test`
