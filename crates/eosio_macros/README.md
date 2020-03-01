[![Build Status](https://travis-ci.org/sagan-software/eosio-rust.svg?branch=master)](https://travis-ci.org/sagan-software/eosio-rust)

# eosio_macros

Macros for creating compile-time EOSIO names and symbols.

Creating EOSIO names:

```rust
use eosio_macros::n;
assert_eq!(n!("test"), 14_605_613_396_213_628_928);
assert_eq!(n!("1234"), 614_248_767_926_829_056);
assert_eq!(n!("123451234512"), 614_251_535_012_020_768);
assert_eq!(n!("eosio.token"), 6_138_663_591_592_764_928);
```

Creating EOSIO symbols:

```rust
use eosio_macros::s;
assert_eq!(s!(4, "EOS"), 1397703940);
```

License: MIT OR Apache-2.0
