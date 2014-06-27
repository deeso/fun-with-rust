#![allow(non_camel_case_types)]

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


pub enum Result {
	R_FAIL,
	R_FALSE,
	R_TRUE,
}

#[link(name="r_core")]
extern "C" {
	pub fn r_core_new () -> *RCore;
	pub fn r_core_init(core: *RCore) -> c_int;
	pub fn r_core_file_open(core: *RCore, file: *c_char, mode : c_int, loadaddr : ut64 ) -> *c_void;
	pub fn r_core_bin_load(core: *RCore, file: *c_char, baseaddr : ut64) -> c_int;
	pub fn r_core_cmd(core: *RCore, cmd: *c_char, log: c_int) -> c_int;
	pub fn r_core_free (core :*RCore) -> *RCore;
}