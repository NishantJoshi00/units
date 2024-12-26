#[repr(C)]
pub struct CString {
    ptr: *const u8,
    len: usize,
}

extern "C" {
    fn get(key_ptr: *const u8, key_len: usize) -> CString;
    fn set(key_ptr: *const u8, key_len: usize, value_ptr: *const u8, value_len: usize);
}

#[no_mangle]
pub extern "C" fn intend(ptr: *const u8, len: usize) -> CString {
    todo!()
}

#[no_mangle]
pub extern "C" fn done(ptr: *const u8, len: usize) {
    todo!()
}

#[no_mangle]
pub extern "C" fn view(ptr: *const u8, len: usize) -> CString {
    todo!()
}

#[no_mangle]
pub extern "C" fn transfer(
    data1_ptr: *const u8,
    len1_ptr: usize,
    data2_ptr: *const u8,
    len2_ptr: usize,
    data3_ptr: *const u8,
    len3_ptr: usize,
) {
    todo!()
}
