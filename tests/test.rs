#![cfg(windows)]
extern crate string_const;

use string_const::{ATTRIB_TEST, RAW_ATTRIB_TEST, RAW_TEST, TEST};

#[test]
fn test() {
    use std::ptr::null_mut;
    assert_eq!(TEST.len(), 15); // Null should not be included
    assert_eq!(RAW_TEST.len(), 28); // Null should not be included
    assert_eq!(ATTRIB_TEST.len(), 22); // Null should not be included
    assert_eq!(RAW_ATTRIB_TEST.len(), 35); // Null should not be included
    eprintln!("{}", TEST);
    eprintln!("{}", RAW_TEST);
    eprintln!("{}", ATTRIB_TEST);
    eprintln!("{}", RAW_ATTRIB_TEST);
    unsafe {
        MessageBoxW(null_mut(), TEST.as_ptr(), TEST.as_ptr(), 0);
        MessageBoxW(null_mut(), RAW_TEST.as_ptr(), RAW_TEST.as_ptr(), 0);
        MessageBoxW(null_mut(), ATTRIB_TEST.as_ptr(), ATTRIB_TEST.as_ptr(), 0);
        MessageBoxW(null_mut(), RAW_ATTRIB_TEST.as_ptr(), RAW_ATTRIB_TEST.as_ptr(), 0);
    }
}

#[link(name = "user32")]
extern "system" {
    pub fn MessageBoxW(
        hwnd: *mut _HWND,
        text: *const u16,
        caption: *const u16,
        _type: u32,
    ) -> i32;
}

#[repr(C)]
pub struct _HWND([u8; 0]);
