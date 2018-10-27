use crate::lib::String;

pub trait Print {
    fn print(&self);
}

impl Print for u8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Print for u16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Print for u32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

impl Print for u64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self) }
    }
}

impl Print for i8 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Print for i16 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Print for i32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

impl Print for i64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self) }
    }
}

impl<'a> Print for &'a str {
    fn print(&self) {
        let ptr = self.as_ptr();
        let len = self.len() as u32;
        unsafe { ::eosio_sys::prints_l(ptr, len) }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Print for String {
    fn print(&self) {
        self.as_str().print()
    }
}

impl Print for bool {
    fn print(&self) {
        let out = if *self { "true" } else { "false" };
        unsafe { ::eosio_sys::prints(out.as_ptr()) }
    }
}

impl Print for usize {
    fn print(&self) {
        (*self as u32).print()
    }
}

impl Print for f32 {
    fn print(&self) {
        unsafe { ::eosio_sys::printsf(*self) }
    }
}

impl Print for f64 {
    fn print(&self) {
        unsafe { ::eosio_sys::printdf(*self) }
    }
}

impl Print for char {
    fn print(&self) {
        let num = *self as u8;
        let ptr = &num as *const ::eosio_sys::c_char;
        unsafe { ::eosio_sys::prints_l(ptr, 1) }
    }
}

impl<P> Print for Option<P>
where
    P: Print,
{
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
