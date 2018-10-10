#![feature(int_to_from_bytes)]

extern "C" {
    pub fn prints_l(cstr: *const u8, len: u32);
    pub fn printn(name: u64);
    pub fn read_action_data(msg: *mut c_void, len: u32) -> u32;
}

#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#[no_mangle]
pub extern "C" fn apply(receiver: u64, code: u64, action: u64) {
    let msg = "Hi, ";
    let msg_ptr = msg.as_ptr();
    let msg_len = msg.len() as u32;
    unsafe { prints_l(msg_ptr, msg_len) };

    let mut name_bytes = [0u8; 8];
    let name_ptr: *mut c_void = &mut name_bytes[..] as *mut _ as *mut c_void;
    unsafe { read_action_data(name_ptr, 8) };

    let name = u64::from_le_bytes(name_bytes);
    unsafe { printn(name) };
}
