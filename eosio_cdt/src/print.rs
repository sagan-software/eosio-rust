use eosio_core::{
    AccountName, ActionName, Name, PermissionName, ScopeName, TableName,
};

pub trait Print {
    fn print(&self);
}

impl Print for u8 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printui(u64::from(*self)) }
    }
}

impl Print for u16 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printui(u64::from(*self)) }
    }
}

impl Print for u32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printui(u64::from(*self)) }
    }
}

impl Print for u64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printui(*self) }
    }
}

impl Print for i8 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printi(i64::from(*self)) }
    }
}

impl Print for i16 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printi(i64::from(*self)) }
    }
}

impl Print for i32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printi(i64::from(*self)) }
    }
}

impl Print for i64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printi(*self) }
    }
}

impl<'a> Print for &'a str {
    #[inline]
    fn print(&self) {
        let ptr = self.as_ptr();
        let len = self.len() as u32;
        unsafe { ::eosio_cdt_sys::prints_l(ptr, len) }
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
        unsafe { ::eosio_cdt_sys::prints(out.as_ptr()) }
    }
}

impl Print for usize {
    #[inline]
    fn print(&self) {
        (*self as u32).print()
    }
}

impl Print for f32 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printsf(*self) }
    }
}

impl Print for f64 {
    #[inline]
    fn print(&self) {
        unsafe { ::eosio_cdt_sys::printdf(*self) }
    }
}

impl Print for char {
    #[inline]
    fn print(&self) {
        let num = *self as u8;
        let ptr = &num as *const ::eosio_cdt_sys::c_char;
        unsafe { ::eosio_cdt_sys::prints_l(ptr, 1) }
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
                unsafe { ::eosio_cdt_sys::printn(self.as_u64()) }
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
