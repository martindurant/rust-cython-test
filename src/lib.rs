use std::ffi::CStr;

static mut DATA: Option<Vec<String>> = None;

#[no_mangle]
pub extern fn init() -> bool {
    unsafe {match DATA {
        None => {
            DATA = Some(Vec::new());
            true
        },
        _ => false
    }}
}

#[no_mangle]
pub extern "C" fn my_upper(b: *const i8) -> *const u8 {
    let s = unsafe { CStr::from_ptr(b) };
    s.to_str().unwrap().to_uppercase().as_ptr()
}
