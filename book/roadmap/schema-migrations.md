# Schema Migrations

_Tracking this feature in [issue #7](https://github.com/sagan-software/eosio-rust/issues/7)_

Making changes to EOS table fields is currently not a pleasant experience. It can be a fragile error-prone process that involves duplicating code to work with multiple versions of structs. We believe that a better solution can be found by taking inspiration from projects like [Diesel](http://diesel.rs/) and [Django migrations](https://docs.djangoproject.com/en/2.1/topics/migrations/).

Implementing this feature will require significant effort and discovery. This may be a 1.0+ feature.
