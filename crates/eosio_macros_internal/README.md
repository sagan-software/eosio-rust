# eosio_macros_internal

Internal use only.

This crate is an implementation detail that will hopefully go away
once the [`proc_macro_hygiene`] feature is stabilized. In the meantime
we must use this crate (and [`proc_macro_hack`]) to allow for
function-like procedural macros in expression positions.

[`proc_macro_hygiene`]: https://doc.rust-lang.org/beta/unstable-book/language-features/proc-macro-hygiene.html
[`proc_macro_hack`]: https://github.com/dtolnay/proc-macro-hack

License: MIT OR Apache-2.0
