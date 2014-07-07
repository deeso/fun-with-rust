extern crate libc;
use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::{c_double, c_float, c_int, c_uint, c_long, c_short, c_uchar, c_ushort};
use libc::types::os::arch::c99::{c_longlong, c_ulonglong};

use std::ptr;

pub mod error;
mod ffi_rcore;

#[cfg(test)]
mod tests;

pub struct RCore {
	core: *mut ffi_rcore::RCore,
	owned: bool
}

impl Drop for RCore {
	fn drop(&mut self) {
		if self.owned {
			unsafe {
				ffi_rcore::r_core_free(self.core);
			}
		}
	}
}

impl RCore {

	pub fn try_new () -> Result<RCore, error::RCoreError> {
		let core = unsafe { ffi_rcore::r_core_new()};
		if core.is_null() {
			return Err(error::FailedToInitCore);
		}
		let rcore = RCore { core: core, owned: false };
		Ok(rcore)
	}

    pub fn new() -> RCore {
        match RCore::try_new() {
            Ok(core) => core,
            Err(err) => fail!("Error creating RCore: {}", err)
        }
    }

    pub fn r_core_file_open(&mut self, file: &str, mode : int, loadaddr : u64 ) -> Option<error::RCoreError>  {
        let ret = file.with_c_str(|file| {
            unsafe {
                ffi_rcore::r_core_file_open(self.core, file, mode as c_int, loadaddr as ffi_rcore::ut64)
            }
        });

        if (ret as c_int) == ffi_rcore::R_FAIL {
            Some(error::FailedToOpenFile)
        } else {
            None
        }
    }

    pub fn r_core_bin_load(&mut self, file: &str, baseaddr : u64 ) -> Option<error::RCoreError>  {
        let ret = file.with_c_str(|file| {
            unsafe {
                ffi_rcore::r_core_bin_load(self.core, file, baseaddr as ffi_rcore::ut64)
            }
        });

        if (ret as c_int) == ffi_rcore::R_FAIL {
            Some(error::FailedToOpenFile)
        } else {
            None
        }
    }

    pub fn r_core_cmd(&mut self, cmd: &str, log: int) -> Option<error::RCoreError> {
        let ret = cmd.with_c_str(|file| {
            unsafe {
                ffi_rcore::r_core_cmd(self.core, file, log as c_int)
            }
        });
        if (ret as c_int) == ffi_rcore::R_FAIL {
            Some(error::FailedCommandExec)
        } else {
            None
        }
    }
}




