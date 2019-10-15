# Serde Support

_Tracking this feature in [issue #11](https://github.com/sagan-software/eosio-rust/issues/11)_

Serde is the defacto standard when it comes to serializing and deserializing data. It will be necessary for table structs to support Serde's `Serialize`/`Deserialize` traits in order to implement the RPC API later on.

Implementing this will require writing custom serializers/deserializers for EOS types, for example:

-   Booleans are 0 or 1
-   Large numbers can sometimes be integers, sometimes be strings
