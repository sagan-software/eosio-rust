pub trait Print {
    fn print(&self);
}

#[cfg(feature = "contract")]
impl Print for u8 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for u16 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for u32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printui(u64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for u64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printui(*self) }
    }
}

#[cfg(feature = "contract")]
impl Print for i8 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for i16 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for i32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printi(i64::from(*self)) }
    }
}

#[cfg(feature = "contract")]
impl Print for i64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printi(*self) }
    }
}

#[cfg(feature = "contract")]
impl<'a> Print for &'a str {
    #[inline]
    fn print(&self) {
        let ptr = self.as_ptr();
        let len = self.len() as u32;
        unsafe { ::eosio_sys::prints_l(ptr, len) }
    }
}

#[cfg(feature = "contract")]
#[cfg(any(feature = "std", feature = "alloc"))]
impl<'a> Print for crate::lib::String {
    #[inline]
    fn print(&self) {
        self.as_str().print()
    }
}

#[cfg(feature = "contract")]
impl Print for bool {
    #[inline]
    fn print(&self) {
        let out = if *self { "true" } else { "false" };
        unsafe { ::eosio_sys::prints(out.as_ptr()) }
    }
}

#[cfg(feature = "contract")]
impl Print for usize {
    #[inline]
    fn print(&self) {
        (*self as u32).print()
    }
}

#[cfg(feature = "contract")]
impl Print for f32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printsf(*self) }
    }
}

#[cfg(feature = "contract")]
impl Print for f64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_sys::printdf(*self) }
    }
}

#[cfg(feature = "contract")]
impl Print for char {
    #[inline]
    fn print(&self) {
        let num = *self as u8;
        let ptr = &num as *const ::eosio_sys::c_char;
        unsafe { ::eosio_sys::prints_l(ptr, 1) }
    }
}

#[cfg(feature = "contract")]
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
