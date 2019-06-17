This crate provides three derive macros for [`eosio_bytes`] traits.

## Examples

```rust
use eosio_bytes::{Read, Write, NumBytes};

#[derive(Read, Write, NumBytes, PartialEq, Debug)]
#[eosio_bytes_root_path = "::eosio_bytes"]
struct Thing(u8);

let thing = Thing(30);

// Number of bytes
assert_eq!(thing.num_bytes(), 1);

// Read bytes
assert_eq!(thing, Thing::read(&mut [30_u8], &mut 0).unwrap());

// Write bytes
let mut bytes = vec![0_u8; 1];
thing.write(&mut bytes, &mut 0).unwrap();
assert_eq!(vec![30], bytes);
```

[`eosio_bytes`]: https://crates.io/crates/eosio_bytes
