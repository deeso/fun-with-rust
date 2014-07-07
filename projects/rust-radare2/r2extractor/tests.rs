#![allow(dead_code)]
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_char, c_double, c_float, c_int, c_uint, c_long, c_short, c_uchar, c_ushort};
use libc::types::os::arch::c99::{c_longlong, c_ulonglong};

use std::ptr;
use std::str;

use error;



#[test]
fn test_new_rcore() {
    RCore::try_new();
}
