
use std::libc::{c_int, c_void};
use std::io::println;


enum RCore {}

[#link_args = "-lr_core"]
extern {
	fn r_core_new () -> *RCore;
	fn r_core_init(core: *RCore) -> c_int;
	fn r_core_file_open(core: *RCore, file: &'static str, mode : c_int, loadaddr : u64 ) -> *c_void;
	fn r_core_bin_load(core: *RCore, file: &'static str, baseaddr : u64) -> c_int;
	fn r_core_cmd(core: *RCore, cmd: &'static str, log: int) -> c_int;
	fn r_core_free (core :*RCore) -> *RCore;
}


pub struct state {
	core : *RCore,
}