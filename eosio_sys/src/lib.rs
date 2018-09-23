#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(alloc)]

extern crate alloc;
extern crate memchr;

mod bindings;
mod cstr;

pub use self::bindings::*;
pub use self::ctypes::*;

pub mod ctypes {
    pub use bindings::{int128_t, uint128_t};
    pub use cstr::*;
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
