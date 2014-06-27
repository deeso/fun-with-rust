#![allow(non_camel_case_types)]
#![allow(dead_code)]
//use crate libc;
//extern crate libc;
//use libc::{c_int, c_void};
use libc::{c_void, c_int, c_uint, c_short, c_ushort, c_uchar, c_char};
use libc::types::os::arch::c99;


pub type RCore = c_void;
pub type ut64 = c99::c_ulonglong;
pub type st64 = c99::c_longlong;
pub type ut32 = c_uint;
pub type st32 = c_int;
pub type ut16 = c_ushort;
pub type st16 = c_short;
pub type ut8 = c_uchar;
pub type st8 = c_char;
pub type boolt = c_int;


pub static R_FAIL : c_int = -1;
pub static R_FALSE : c_int = 0;
pub static R_TRUE : c_int = 1;

pub static UT64_MAX : c99::c_ulonglong = 0xFFFFFFFFFFFFFFFF;
pub static UT64_GT0 : c99::c_ulonglong = 0x8000000000000000;
pub static UT64_LT0 : c99::c_ulonglong = 0x7FFFFFFFFFFFFFFF;
pub static UT64_MIN : c99::c_ulonglong = 0;
pub static UT64_32U : c99::c_ulonglong = 0xFFFFFFFF00000000;
pub static UT64_16U : c99::c_ulonglong = 0xFFFFFFFFFFFF0000;
pub static UT64_8U : c99::c_ulonglong =  0xFFFFFFFFFFFFFF00;

pub static UT32_MIN : c_uint = 0;
pub static UT32_MAX : c_uint = 0xFFFFFFFF;
pub static UT32_GT0 : c_uint = 0x80000000;
pub static UT32_LT0 : c_uint = 0x7FFFFFFF;

pub static UT32_24U : c_uint = 0xFF000000;
pub static UT32_16U : c_uint = 0xFFFF0000;
pub static UT32_8U : c_uint =  0xFFFFFF00;
pub static ST32_MAX : c_int = 0x7FFFFFFF;
pub static ST32_MIN : c_int = -1;

pub static UT16_GT0 : c_ushort = 0x8000;
pub static UT16_MAX : c_ushort = 0xFFFF;
pub static UT16_8U : c_uint =  0xFF00;
pub static ST16_MAX : c_short = 0x7FFF;
pub static ST16_MIN : c_short = -1;

pub static UT8_GT0 : c_uchar = 0x80;
pub static UT8_MAX : c_uchar = 0xFF;
pub static UT8_MIN : c_uchar = 0x00;


#[link(name="r_core")]
extern "C" {
	pub fn r_core_new () -> *RCore;
	pub fn r_core_init(core: *RCore) -> c_int;
	pub fn r_core_file_open(core: *RCore, file: *c_char, mode : c_int, loadaddr : ut64 ) -> *c_void;
	pub fn r_core_bin_load(core: *RCore, file: *c_char, baseaddr : ut64) -> c_int;
	pub fn r_core_cmd(core: *RCore, cmd: *c_char, log: c_int) -> c_int;
	pub fn r_core_free (core :*RCore) -> *RCore;
}

#[link(name="r_cons")]
extern "C" {
	pub fn r_cons_flush ();
}