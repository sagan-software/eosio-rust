[![Build Status](https://travis-ci.org/sagan-software/eosio-rust.svg?branch=master)](https://travis-ci.org/sagan-software/eosio-rust)

# eosio_numstr

This crate provides functions for converting EOSIO names and
symbols to and from string representations.

Creating an EOSIO name:

```rust
use eosio_numstr::name_from_bytes;
let name = name_from_bytes("eosio".bytes()).unwrap();
assert_eq!(name, 6138663577826885632);
```

Creating an EOSIO symbol:

```rust
use eosio_numstr::symbol_from_bytes;
let symbol = symbol_from_bytes(4, "EOS".bytes()).unwrap();
assert_eq!(symbol, 1397703940);
```

License: MIT OR Apache-2.0
