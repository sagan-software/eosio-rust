use alloc::{string::String, vec::Vec};
use eosio::{
    AccountName, ActionName, Checksum160, Checksum256, Checksum512, Name,
    PermissionName, PublicKey, ScopeName, Signature, TableName, TimePoint,
    TimePointSec,
};
use eosio_cdt_sys::{
    c_void, printdf, printhex, printi, printn, prints, prints_l, printsf,
    printui,
};

/// Trait for types that can be printed from within EOSIO smart contracts
pub trait Print {
    /// Print to the console
    fn print(&self);
}

impl Print for u8 {
    #[inline]
    fn print(&self) {
        unsafe { printui(u64::from(*self)) }
    }
}

impl Print for u16 {
    #[inline]
    fn print(&self) {
        unsafe { printui(u64::from(*self)) }
    }
}

impl Print for u32 {
    #[inline]
    fn print(&self) {
        unsafe { printui(u64::from(*self)) }
    }
}

impl Print for u64 {
    #[inline]
    fn print(&self) {
        unsafe { printui(*self) }
    }
}

impl Print for i8 {
    #[inline]
    fn print(&self) {
        unsafe { printi(i64::from(*self)) }
    }
}

impl Print for i16 {
    #[inline]
    fn print(&self) {
        unsafe { printi(i64::from(*self)) }
    }
}

impl Print for i32 {
    #[inline]
    fn print(&self) {
        unsafe { printi(i64::from(*self)) }
    }
}

impl Print for i64 {
    #[inline]
    fn print(&self) {
        unsafe { printi(*self) }
    }
}

impl<'a> Print for &'a str {
    #[inline]
    fn print(&self) {
        let ptr = self.as_ptr();
        #[allow(clippy::cast_possible_truncation)]
        let len = self.len() as u32;
        unsafe { prints_l(ptr, len) }
    }
}

impl<'a> Print for String {
    #[inline]
    fn print(&self) {
        self.as_str().print()
    }
}

impl Print for bool {
    #[inline]
    fn print(&self) {
        let out = if *self { "true" } else { "false" };
        unsafe { prints(out.as_ptr()) }
    }
}

impl Print for usize {
    #[inline]
    fn print(&self) {
        (*self as u64).print()
    }
}

impl Print for f32 {
    #[inline]
    fn print(&self) {
        unsafe { printsf(*self) }
    }
}

impl Print for f64 {
    #[inline]
    fn print(&self) {
        unsafe { printdf(*self) }
    }
}

impl Print for char {
    #[inline]
    fn print(&self) {
        let num = *self as u8;
        let ptr = &num as *const u8;
        unsafe { prints_l(ptr, 1) }
    }
}

impl<P> Print for Option<P>
where
    P: Print,
{
    #[inline]
    fn print(&self) {
        match self {
            Some(p) => {
                "Some(".print();
                p.print();
                ")".print();
            }
            None => "None".print(),
        }
    }
}

macro_rules! impl_print_for_names {
    ($($ident:ident)*) => ($(
        #[automatically_derived]
        impl Print for $ident {
            #[inline]
            fn print(&self) {
                unsafe { printn(self.as_u64()) }
            }
        }
    )*)
}

impl_print_for_names! {
    Name
    AccountName
    PermissionName
    ScopeName
    TableName
    ActionName
}

impl Print for TimePoint {
    #[inline]
    fn print(&self) {
        "TimePoint(".print();
        self.as_micros().print();
        ")".print();
    }
}

impl Print for TimePointSec {
    #[inline]
    fn print(&self) {
        "TimePointSec(".print();
        self.as_secs().print();
        ")".print();
    }
}

/// Prints arbitrary data to the nodeos console
#[macro_export]
macro_rules! print {
    ($e:expr) => (
        $crate::Print::print(&$e);
    );
    ($e:expr, $($es:expr),+) => (
        $crate::print!($e);
        $crate::print!($($es),+);
    );
}

impl Print for &[u8] {
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn print(&self) {
        let ptr = self.as_ptr() as *const c_void;
        let len = self.len();
        unsafe { printhex(ptr, len as u32) }
    }
}

macro_rules! impl_print_for_as_slice_types {
    ($($ident:ty)*) => ($(
        #[automatically_derived]
        impl Print for $ident {
            #[inline]
            fn print(&self) {
                self.as_slice().print()
            }
        }
    )*)
}

impl_print_for_as_slice_types! {
    Vec<u8>
    PublicKey
    Signature
}

macro_rules! impl_print_for_checksum_types {
    ($($ident:ty)*) => ($(
        #[automatically_derived]
        impl Print for $ident {
            #[inline]
            fn print(&self) {
                let bytes = self.to_bytes();
                (&bytes).print();
            }
        }
    )*)
}

impl_print_for_checksum_types! {
    Checksum160
    Checksum256
    Checksum512
}

macro_rules! impl_print_for_byte_arrays {
    ($($bytes:literal)*) => ($(
        #[automatically_derived]
        impl Print for [u8; $bytes] {
            #[inline]
            fn print(&self) {
                (&self[..]).print()
            }
        }
    )*)
}

impl_print_for_byte_arrays! {
     1  2  3  4  5  6  7  8
     9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24
    25 26 27 28 29 30 31 32
    33 34 35 36 37 38 39 40
    41 42 43 44 45 46 47 48
    49 50 51 52 53 54 55 56
    57 58 59 60 61 62 63 64
}
