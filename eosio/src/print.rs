use eosio_macros::c;
use lib::String;

pub trait Printable {
    fn print(&self);
}

impl Printable for u8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Printable for u16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Printable for u32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Printable for u64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self) }
    }
}

impl Printable for i8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Printable for i16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Printable for i32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Printable for i64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self) }
    }
}

impl<'a> Printable for &'a str {
    fn print(&self) {
        // TODO: Make sure &self is a C string (ends with \0)
        unsafe { ::eosio_sys::prints(self.as_ptr()) }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Printable for String {
    fn print(&self) {
        // TODO: Make sure &self is a C string (ends with \0)
        self.as_str().print()
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

impl Printable for char {
    fn print(&self) {
        let num = *self as u8;
        let ptr = &num as *const ::eosio_sys::c_char;
        unsafe { ::eosio_sys::prints_l(ptr, 1) }
    }
}
