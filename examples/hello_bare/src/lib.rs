extern "C" {
    pub fn prints_l(cstr: *const u8, len: u32);
    pub fn printn(name: u64);
    pub fn read_action_data(msg: *mut CVoid, len: u32) -> u32;
}

#[repr(u8)]
pub enum CVoid {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)]
    Variant1,
    #[doc(hidden)]
    Variant2,
}

#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, _action: u64) {
    let msg = "Hi, ";
    let msg_ptr = msg.as_ptr();
    let msg_len = msg.len() as u32;
    unsafe { prints_l(msg_ptr, msg_len) };

    let mut name_bytes = [0u8; 8];
    let name_ptr: *mut CVoid = &mut name_bytes[..] as *mut _ as *mut CVoid;
    unsafe { read_action_data(name_ptr, 8) };

    // let name = u64::from_le_bytes(name_bytes);
    // unsafe { printn(name) };
}
