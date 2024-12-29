mod safe_driver;

#[link(wasm_import_module = "platform")]
extern "C" {
    fn get(key_ptr: *const u8, key_len: usize) -> (*const u8, usize);
    fn set(key_ptr: *const u8, key_len: usize, value_ptr: *const u8, value_len: usize);
}

#[no_mangle]
pub unsafe extern "C" fn intend(ptr: *const u8, len: usize) -> (*const u8, usize) {
    safe_driver::safe_intend(Ptr::from_wasmptr(ptr, len)).as_wasmptr()
}

#[no_mangle]
pub unsafe extern "C" fn done(ptr: *const u8, len: usize) {
    safe_driver::safe_done(Ptr::from_wasmptr(ptr, len));
}

#[no_mangle]
pub unsafe extern "C" fn view(ptr: *const u8, len: usize) -> (*const u8, usize) {
    safe_driver::safe_view(Ptr::from_wasmptr(ptr, len)).as_wasmptr()
}

#[no_mangle]
pub unsafe extern "C" fn transfer(
    from_ptr: *const u8,
    from_len: usize,
    to_ptr: *const u8,
    to_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) {
    safe_driver::safe_transfer(
        Ptr::from_wasmptr(from_ptr, from_len),
        Ptr::from_wasmptr(to_ptr, to_len),
        Ptr::from_wasmptr(data_ptr, data_len),
    );
}

pub(crate) trait Ptr {
    unsafe fn from_wasmptr(ptr: *const u8, len: usize) -> Self;
    fn as_wasmptr(&self) -> (*const u8, usize);
}

impl Ptr for String {
    fn as_wasmptr(&self) -> (*const u8, usize) {
        (self.as_ptr(), self.len())
    }

    unsafe fn from_wasmptr(ptr: *const u8, len: usize) -> Self {
        let input = unsafe { std::slice::from_raw_parts(ptr, len) };
        let s = std::str::from_utf8(input).unwrap();
        s.to_string()
    }
}

impl<'a> Ptr for &'a str {
    fn as_wasmptr(&self) -> (*const u8, usize) {
        ((*self).as_ptr(), self.len())
    }

    unsafe fn from_wasmptr(ptr: *const u8, len: usize) -> Self {
        let input = std::slice::from_raw_parts(ptr, len);
        let s = std::str::from_utf8(input).unwrap();
        s
    }
}
