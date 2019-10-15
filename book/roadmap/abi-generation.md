# ABI Generation

_Tracking this feature in [issue #5](https://github.com/sagan-software/eosio-rust/issues/5)_

Hand-written ABI files are unnecessary and expose developers to risk if they aren't kept updated.

Since we already have `#[eosio::action]` and `#[eosio::table]` attributes, it should be fairly straightforward to implement this feature by detecting these attributes and generating a JSON file.
