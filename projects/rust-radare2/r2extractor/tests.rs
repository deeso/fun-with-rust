#![allow(dead_code)]
use libc::types::common::c95::{c_void};
use libc::types::os::arch::c95::{c_char, c_double, c_float, c_int, c_uint, c_long, c_short, c_uchar, c_ushort};
use libc::types::os::arch::c99::{c_longlong, c_ulonglong};

use std::ptr;
use std::str;

use error;
use RCore;


#[test]
fn test_new_rcore() {
    RCore::try_new();
}

#[test]
fn test_new_rcore_file_open() {
    let mut core : RCore = match RCore::try_new() {
        Ok(core) => core,
        Err(err) => fail!("Expected a valid core structure back")
    };
	//let filename = "/bin/ls"; //when this is commented out it works
}
