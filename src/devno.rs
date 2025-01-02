// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CStr;

use crate::err::Result;

/// Device number
pub struct BlkidDevno(libc::dev_t);

#[cfg(target_os = "linux")]
pub mod ty {
    /// Type alias for device major number
    #[allow(non_camel_case_types)]
    pub type maj_t = libc::c_uint;

    /// Type alias for device minor number
    #[allow(non_camel_case_types)]
    pub type min_t = libc::c_uint;
}

#[cfg(all(unix, not(target_os = "linux"), not(target_os = "android")))]
pub mod ty {
    /// Type alias for device major number
    #[allow(non_camel_case_types)]
    pub type maj_t = i32;

    /// Type alias for device minor number
    #[allow(non_camel_case_types)]
    pub type min_t = i32;
}

pub use ty::*;

impl BlkidDevno {
    pub(crate) fn new(devno: libc::dev_t) -> Self {
        BlkidDevno(devno)
    }

    pub(crate) fn as_dev_t(&self) -> libc::dev_t {
        self.0
    }

    /// Create a `BlkidDevno` from major and minor numbers.
    pub fn from_device_numbers(major: maj_t, minor: min_t) -> Self {
        #[allow(unused_unsafe, reason = "No longer unsafe in libc 0.2.133")]
        BlkidDevno(unsafe { libc::makedev(major, minor) })
    }

    /// Get the major number.
    pub fn major(&self) -> maj_t {
        #[allow(unused_unsafe, reason = "No longer unsafe in libc 0.2.171")]
        unsafe {
            libc::major(self.0)
        }
    }

    /// Get the minor number.
    pub fn minor(&self) -> min_t {
        #[allow(unused_unsafe, reason = "No longer unsafe in libc 0.2.171")]
        unsafe {
            libc::minor(self.0)
        }
    }

    /// Get device name from device number
    pub fn to_devname(&self) -> Result<String> {
        let ret = errno_ptr!(unsafe { libblkid_rs_sys::blkid_devno_to_devname(self.0) })?;
        let string_ret = unsafe { CStr::from_ptr(ret) }.to_str()?.to_string();
        unsafe { libc::free(ret as *mut libc::c_void) };
        Ok(string_ret)
    }

    /// Get the device number and name of the whole disk associated with this device
    /// number
    pub fn to_wholedisk(&self) -> Result<(String, BlkidDevno)> {
        let buf = &mut [0u8; 4096];
        let mut wholedisk_devno: libc::dev_t = 0;
        errno!(unsafe {
            libblkid_rs_sys::blkid_devno_to_wholedisk(
                self.0,
                buf.as_mut_ptr() as *mut libc::c_char,
                buf.len(),
                &mut wholedisk_devno as *mut _,
            )
        })?;
        let name = std::str::from_utf8(buf)?.to_string();
        Ok((name, BlkidDevno(wholedisk_devno)))
    }
}
