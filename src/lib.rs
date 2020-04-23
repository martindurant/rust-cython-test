use std::slice;
use std::str;
use std::ptr;

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
pub extern "C" fn ak_upper(
    contents: *const u8, offsets: *const i64, mut out: *mut u8, n: usize, size: usize) {
    let inarr = unsafe { slice::from_raw_parts(contents, size) };
    let offs =  unsafe { slice::from_raw_parts(offsets, n) };
    let mut last_off = offs[0] as usize;
    let mut count: usize;
    for i in 1..n {
        let this_off = offs[i] as usize;
        count = this_off - last_off;
        unsafe {let s = str::from_utf8_unchecked(&inarr[last_off .. this_off]);
        ptr::copy_nonoverlapping(
            s.to_uppercase().as_ptr(),
            out,
            count
        );
            out = out.offset(count as isize);
        }
        last_off = this_off;
    };
}

#[no_mangle]
pub extern "C" fn ak_bupper(
    contents: *const u8, mut out: *mut u8, size: usize) {
    let inarr = unsafe { slice::from_raw_parts(contents, size) };
    for i in 1..size {
        unsafe {
            *out = inarr[i].to_ascii_uppercase();
            out = out.offset(1);
        }
    };
}

fn cmp(in1: &[u8], in2: &[u8], count: usize) -> u8 {
    for i in 0..count {
        if in1[i] != in2[i] {
            return 0;
        }
    }
    1
}

#[no_mangle]
pub extern "C" fn ak_eq(
    contents1: *const u8, offsets1: *const usize, size1: usize,
    contents2: *const u8, offsets2: *const usize, size2: usize,
    n: usize,
    mut out: *mut u8
) {
    let arr1 = unsafe { slice::from_raw_parts(contents1, size1) };
    let arr2 = unsafe { slice::from_raw_parts(contents2, size2) };
    let offs1 =  unsafe { slice::from_raw_parts(offsets1, n + 1) };
    let offs2 =  unsafe { slice::from_raw_parts(offsets2, n + 1) };
    let mut count1: usize;
    let mut count2: usize;
    for i in 1..n+1 {
        count1 = offs1[i] - offs1[i - 1];
        count2 = offs2[i] - offs2[i - 1];
        unsafe {*out = {
            if count1 == count2 {cmp(
                &arr1[offs1[i - 1]..offs1[i]],
                &arr2[offs2[i - 1]..offs2[i]],
                count1
                )
            } else { 0 }
        };
            out = out.offset(1);
        }
    }
}
