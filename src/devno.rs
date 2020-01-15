use std::ffi::CStr;

use crate::err::Result;

/// Device number
pub struct BlkidDevno(libc::dev_t);

impl BlkidDevno {
    /// Get device name from device number
    pub fn to_devname(&self) -> Result<&str> {
        let ret = errno_ptr!(unsafe { libblkid_rs_sys::blkid_devno_to_devname(self.0) })?;
        let cstr_ret = unsafe { CStr::from_ptr(ret) };
        Ok(cstr_ret.to_str()?)
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
