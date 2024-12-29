#[link(wasm_import_module = "driver")]
extern "C" {
    fn intend(path_ptr: *const u8, path_len: usize) -> (*const u8, usize);
    fn done(path_ptr: *const u8, path_len: usize);
    fn transfer(
        path_ptr1: *const u8,
        path_len1: usize,
        path_ptr2: *const u8,
        path_len2: usize,
        path_ptr3: *const u8,
        path_len3: usize,
    );
    fn view(path_ptr: *const u8, path_len: usize) -> (*const u8, usize);
}

mod safe_drivers;

#[no_mangle]
pub unsafe extern "C" fn main(input_ptr: *const u8, input_len: usize) -> (*const u8, usize) {
    safe_drivers::safe_main(Ptr::from_wasmptr(input_ptr, input_len)).as_wasmptr()
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
        let s = std::str::from_utf8(input).expect("invalid utf8");
        s.to_string()
    }
}

impl<'a> Ptr for &'a str {
    fn as_wasmptr(&self) -> (*const u8, usize) {
        ((*self).as_ptr(), self.len())
    }

    unsafe fn from_wasmptr(ptr: *const u8, len: usize) -> Self {
        let input = std::slice::from_raw_parts(ptr, len);
        let s = std::str::from_utf8(input).expect("invalid utf8");
        s
    }
}
