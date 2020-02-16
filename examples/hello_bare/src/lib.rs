// Declare the EOSIO externs to read action data and print to the console.
#[cfg(target_arch = "wasm32")]
extern "C" {
    pub fn read_action_data(msg: *mut CVoid, len: u32) -> u32;
    pub fn prints_l(cstr: *const u8, len: u32);
    pub fn printn(name: u64);
}

#[repr(u8)]
pub enum CVoid {
    // Two dummy variants so the #[repr] attribute can be used.
    Variant1,
    Variant2,
}

// EOSIO smart contracts are expected to have an `apply` function.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn apply(_receiver: u64, _code: u64, _action: u64) {
    // First print "Hi, " to the console.
    {
        let msg = "Hi, ";
        let ptr = msg.as_ptr();
        let len = msg.len() as u32;
        unsafe { prints_l(ptr, len) };
    }

    // Read the action data, which is one EOSIO name (a u64, or 8 bytes).
    let name = {
        let mut bytes = [0u8; 8];
        let ptr: *mut CVoid = &mut bytes[..] as *mut _ as *mut CVoid;
        unsafe { read_action_data(ptr, 8) };
        u64::from_le_bytes(bytes)
    };

    // Finally, print the name to the console.
    unsafe { printn(name) };
}
