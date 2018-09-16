use eosio_macros::cstr;
use eosio_sys::print::*;

pub trait Printable {
    fn print(&self);
}

impl Printable for ::types::Name {
    fn print(&self) {
        unsafe { printn(self.as_u64()) }
    }
}

impl Printable for u8 {
    fn print(&self) {
        unsafe { printui(*self as u64) }
    }
}

impl Printable for u16 {
    fn print(&self) {
        unsafe { printui(*self as u64) }
    }
}

impl Printable for u32 {
    fn print(&self) {
        unsafe { printui(*self as u64) }
    }
}

impl Printable for u64 {
    fn print(&self) {
        unsafe { printui(*self) }
    }
}

impl Printable for i8 {
    fn print(&self) {
        unsafe { printi(*self as i64) }
    }
}

impl Printable for i16 {
    fn print(&self) {
        unsafe { printi(*self as i64) }
    }
}

impl Printable for i32 {
    fn print(&self) {
        unsafe { printi(*self as i64) }
    }
}

impl Printable for i64 {
    fn print(&self) {
        unsafe { printi(*self) }
    }
}

impl<'a> Printable for &'a str {
    fn print(&self) {
        unsafe { prints(self.as_ptr()) }
    }
}

impl Printable for bool {
    fn print(&self) {
        let out = if *self { cstr!("true") } else { cstr!("false") };
        unsafe { prints(out.as_ptr()) }
    }
}

impl Printable for f32 {
    fn print(&self) {
        unsafe { printsf(*self) }
    }
}

impl Printable for f64 {
    fn print(&self) {
        unsafe { printdf(*self) }
    }
}
