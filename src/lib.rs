use std::ffi::CStr;
use std::slice;
use std::str;

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

#[no_mangle]
pub extern "C" fn ak_upper(
    contents: *const u8, offsets: *const u8, out: *mut u8, n: usize, size: usize) {
    let inarr = unsafe { slice::from_raw_parts(contents, size) };
    let offs =  unsafe { slice::from_raw_parts(offsets, n) };
    let outarr = unsafe { slice::from_raw_parts_mut(out, size) };;
    let mut last_off: usize = offs[0] as usize;
    for i in 1..(n-1) {
        let this_off = offs[i] as usize;
        let s = str::from_utf8(&inarr[last_off .. this_off]).unwrap();
        for (chin, chout) in s.to_uppercase().as_bytes().iter().zip(
            outarr[last_off .. this_off].iter_mut()){
            *chout = *chin;
        }
        last_off = this_off;
    }
}
