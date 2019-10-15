# Browser Support

_Tracking this feature in [issue #10](https://github.com/sagan-software/eosio-rust/issues/10)_

A big selling point of Rust is its first-class support for WebAssembly and the possibility of writing full-stack web applications in one highly performant language. It would be great if we could use the same structs and functions from our smart contracts in our frontend code as well.

Implementing this may require rethinking some things, specifically traits that are implemented on primitive types like `SecondaryTableKey` seem to be causing some issues.
