#![allow(dead_code)]
use libc::{c_void, c_int, c_uint, c_short, c_ushort, c_uchar, c_char};
use libc::types::os::arch::c99;
use std::ptr;
use std::str;

use r2extractor::{RCore};
use error;
use ffi_rcore;



#[test]
fn test_new_rcore() {
    RCore::try_new();
}