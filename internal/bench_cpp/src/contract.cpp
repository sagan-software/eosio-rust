#include <eosio/eosio.hpp>
#include <eosio/asset.hpp>
#include <eosio/symbol.hpp>
#include <eosio/crypto.hpp>

using namespace eosio;
using std::string;
using std::vector;

#define TYPE_ACTIONS(SUFFIX, TYPE)                  \
    TABLE t##SUFFIX                                 \
    {                                               \
        uint64_t pk;                                \
        TYPE data;                                  \
        uint64_t primary_key() const { return pk; } \
    };                                              \
    ACTION read##SUFFIX(TYPE data) {}               \
    ACTION write##SUFFIX(uint64_t pk, TYPE data)    \
    {                                               \
    }                                               \
    ACTION chg##SUFFIX(uint64_t pk, TYPE data)      \
    {                                               \
    }                                               \
    ACTION del##SUFFIX(uint64_t pk)                 \
    {                                               \
    }

CONTRACT bench : public contract
{
public:
    using contract::contract;

    ACTION noop(ignore<name> data) {}

    TYPE_ACTIONS(u1, uint8_t)
    TYPE_ACTIONS(u2, uint16_t)
    TYPE_ACTIONS(u3, uint32_t)
    TYPE_ACTIONS(u4, uint64_t)
    TYPE_ACTIONS(u5, uint128_t)
    TYPE_ACTIONS(i1, int8_t)
    TYPE_ACTIONS(i2, int16_t)
    TYPE_ACTIONS(i3, int32_t)
    TYPE_ACTIONS(i4, int64_t)
    TYPE_ACTIONS(i5, int128_t)
    TYPE_ACTIONS(string, string)
    TYPE_ACTIONS(name, name)
    TYPE_ACTIONS(asset, asset)
    TYPE_ACTIONS(easset, extended_asset)
    TYPE_ACTIONS(symbol, symbol)
    TYPE_ACTIONS(esymbl, extended_symbol)
    TYPE_ACTIONS(c1, checksum160)
    TYPE_ACTIONS(c2, checksum256)
    TYPE_ACTIONS(c3, checksum512)
};