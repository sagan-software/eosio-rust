# ABI to Rust

_Tracking this feature in [issue #6](https://github.com/sagan-software/eosio-rust/issues/6)_

It would be nice to have a CLI command that would generate Rust code from on-chain ABIs. This would make it significantly easier to interact with external contracts through inline actions.

Implementing this feature would require fetching the ABI JSON from an EOS node and creating a Rust file containing the generated tables and actions.
