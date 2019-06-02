This crate provides functions for converting EOSIO names and
symbols, represented as `u64`, to and from string representations.

Converting names:

```rust
use eosio_numstr::{name_from_str, name_to_string};

let name = name_from_str("eosio").unwrap();
assert_eq!(name, 6138663577826885632);
assert_eq!(name_to_string(name), "eosio");
```

Converting symbols:

```rust
use eosio_numstr::{symbol_from_str, symbol_to_string};

let symbol = symbol_from_str(4, "EOS").unwrap();
assert_eq!(symbol, 1397703940);
assert_eq!(symbol_to_string(symbol), "EOS");
```
