- [ ] Create concrete `SecondaryKey` type
- [ ] Create concrete `SecondaryKeys` type
- [ ] Create `From<T>` impls for `SecondaryKeys`
- [ ] Move `TableRow` trait to `eosio-core`
- [ ] Move most of `TableRow` functions to new `Table` trait
- [ ] Move generated secondary key functions to `eosio::table` macro


# `eosio_cdt`

- [x] Create test script using test files from `EOSIO/eosio.cdt` repo
- [ ] Create contract benchmarks
  - [ ] Create Rust benchmarking contract
  - [ ] Create C++ benchmarking contract
- [x] Re-implement sending inline actions
- [ ] Implement deferred transactions
- [x] Panic handler
- [x] Singleton

# `eosio_core`

- [ ] `.checked_` functions for `Asset`
  - [ ] `checked_abs`
  - [ ] `checked_add`
  - [ ] `checked_div`
  - [ ] `checked_rem`
  - [ ] `checked_sub`
  - [ ] `checked_pow`
  - [ ] `checked_neg`
- [ ] `std::ops` traits for `Asset`
  - [ ] `Add`
  - [ ] `AddAssign`
  - [ ] `Div`
  - [ ] `DivAssign`
  - [ ] `Mul`
  - [ ] `MulAssign`
  - [ ] `Neg`
  - [ ] `Rem`
  - [ ] `RemAssign`
  - [ ] `Sub`
  - [ ] `SubAssign`
- [ ] `.checked_` functions for `ExtendedAsset`
  - [ ] `checked_abs`
  - [ ] `checked_add`
  - [ ] `checked_div`
  - [ ] `checked_rem`
  - [ ] `checked_sub`
  - [ ] `checked_pow`
  - [ ] `checked_neg`
- [ ] `std::ops` traits for `ExtendedAsset`
  - [ ] `Add`
  - [ ] `AddAssign`
  - [ ] `Div`
  - [ ] `DivAssign`
  - [ ] `Mul`
  - [ ] `MulAssign`
  - [ ] `Neg`
  - [ ] `Rem`
  - [ ] `RemAssign`
  - [ ] `Sub`
  - [ ] `SubAssign`
- [x] `FromStr` for `Asset`
- [ ] `FromStr` for `ExtendedAsset`
- [ ] `FromStr` for `Authorization`
- [ ] `Display` for `Authorization`
- [ ] Create `TimePoint` type
- [ ] Create `TimePointSec` type
- [ ] Create `BlockTimestamp` type
- [ ] Create `PublicKey` type
- [ ] Create `PrivateKey` type
- [ ] Create `Signature` type
- [ ] Create `ParseExtendedAssetError` enum
- [ ] Create `ParseExtendedSymbolError` enum

# `eosio_numstr_macros`

- [ ] UI tests