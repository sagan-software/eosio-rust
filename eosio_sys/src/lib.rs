#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate memchr;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate std;

mod bindings;
// mod cstr;

pub use self::bindings::*;
pub use self::ctypes::*;

pub mod ctypes {
    pub use bindings::{int128_t, uint128_t};
    // pub use cstr::*;
    pub use std::ffi::*;
    pub type c_char = c_uchar;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type int8_t = i8;
    pub type int16_t = i16;
    pub type int32_t = i32;
    pub type int64_t = i64;
    pub type uint8_t = u8;
    pub type uint16_t = u16;
    pub type uint32_t = u32;
    pub type uint64_t = u64;
    pub type c_schar = i8;
    pub type c_short = i16;
    pub type c_longlong = i64;
    pub type c_uchar = u8;
    pub type c_ushort = u16;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type intmax_t = i64;
    pub type uintmax_t = u64;
    pub type size_t = usize;
    pub type ptrdiff_t = isize;
    pub type intptr_t = isize;
    pub type uintptr_t = usize;
    pub type ssize_t = isize;

    #[repr(u8)]
    pub enum c_void {
        // Two dummy variants so the #[repr] attribute can be used.
        #[doc(hidden)]
        __variant1,
        #[doc(hidden)]
        __variant2,
    }
}

fn char_to_symbol(c: char) -> Option<char> {
    if c >= 'a' && c <= 'z' {
        ::std::char::from_u32((c as u32 - 'a' as u32) + 6)
    } else if c >= '1' && c <= '5' {
        ::std::char::from_u32((c as u32 - '1' as u32) + 1)
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum ToNameError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

pub fn string_to_name(s: &str) -> Result<u64, ToNameError> {
    if s.is_empty() {
        return Err(ToNameError::IsEmpty);
    }

    if s.len() > 12 {
        return Err(ToNameError::TooLong);
    }

    let mut value = 0;

    for (i, c) in s.chars().enumerate() {
        match char_to_symbol(c) {
            Some(symbol) => {
                let mut n = symbol as u64;
                if i < 12 {
                    n &= 31u64;
                    n <<= 64 - 5 * (i + 1);
                } else {
                    n &= 15u64;
                }
                value |= n;
            }
            None => {
                return Err(ToNameError::BadChar(c));
            }
        }
    }

    Ok(value)
}

pub const NAME_CHARS: [u8; 32] = *b".12345abcdefghijklmnopqrstuvwxyz";

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn name_to_string(name: u64) -> String {
    let mut chars = [b'.'; 13];
    let mut t = name;
    for i in 0..13 {
        let charmap_index = t & if i == 0 { 15 } else { 31 };
        let mut c = NAME_CHARS[charmap_index as usize];
        chars[12 - i] = c;
        t >>= if i == 0 { 4 } else { 5 };
    }
    ::std::str::from_utf8(&chars)
        .unwrap()
        .trim_matches('.')
        .to_string()
}

#[derive(Debug, PartialEq)]
pub enum ToSymbolError {
    IsEmpty,
    TooLong,
    BadChar(char),
}

pub fn string_to_symbol(precision: u8, s: &str) -> Result<u64, ToSymbolError> {
    let mut result: u64 = 0;
    for (i, c) in s.chars().enumerate() {
        if c < 'A' || c > 'Z' {
            return Err(ToSymbolError::BadChar(c));
        } else {
            result |= (c as u64) << (8 * (i + 1));
        }
    }

    result |= u64::from(precision);
    Ok(result)
}

pub fn symbol_name_length(symbol: u64) -> usize {
    let mut sym = symbol;
    sym >>= 8; // skip precision
    let mut len = 0;
    while sym & 255 > 0 && len <= 7 {
        len += 1;
        sym >>= 8;
    }
    len
}
