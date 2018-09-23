use eosio_macros::c;
use eosio_types::*;

pub trait Printable {
    fn print(&self);
}

impl Printable for Name {
    fn print(&self) {
        unsafe { ::eosio_sys::printn(self.as_u64()) }
    }
}

impl Printable for u8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self as u64) }
    }
}

impl Printable for u16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self as u64) }
    }
}

impl Printable for u32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self as u64) }
    }
}

impl Printable for u64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self) }
    }
}

impl Printable for i8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self as i64) }
    }
}

impl Printable for i16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self as i64) }
    }
}

impl Printable for i32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self as i64) }
    }
}

impl Printable for i64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self) }
    }
}

impl<'a> Printable for &'a str {
    fn print(&self) {
        unsafe { ::eosio_sys::prints(self.as_ptr()) }
    }
}

impl Printable for bool {
    fn print(&self) {
        let out = if *self { c!("true") } else { c!("false") };
        unsafe { ::eosio_sys::prints(out.as_ptr()) }
    }
}

impl Printable for usize {
    fn print(&self) {
        (*self as u32).print()
    }
}

impl Printable for f32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printsf(*self) }
    }
}

impl Printable for f64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printdf(*self) }
    }
}
