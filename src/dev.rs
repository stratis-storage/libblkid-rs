use std::{
    ffi::{CStr, CString},
    ptr,
};

use crate::err::{BlkidErr, Result};

/// Block device found by blkid
pub struct BlkidDev(libblkid_rs_sys::blkid_dev);

impl BlkidDev {
    pub(crate) fn new(inner: libblkid_rs_sys::blkid_dev) -> Self {
        BlkidDev(inner)
    }

    /// Get the device name for a blkid device
    pub fn devname(&self) -> Result<&str> {
        let ret = unsafe { libblkid_rs_sys::blkid_dev_devname(self.0) };
        if ret.is_null() {
            return Err(BlkidErr::LibErr);
        }
        let cstr_ret = unsafe { CStr::from_ptr(ret) };
        Ok(cstr_ret.to_str()?)
    }
}

/// Iterator for blkid-discovered block devices
pub struct BlkidDevIter(libblkid_rs_sys::blkid_dev_iterate);

impl BlkidDevIter {
    pub(crate) fn new(iter: libblkid_rs_sys::blkid_dev_iterate) -> Self {
        BlkidDevIter(iter)
    }

    /// Set search parameters for iteration
    pub fn search(self, search_type: &str, search_value: &str) -> Result<Self> {
        let search_type_cstring = CString::new(search_type)?;
        let search_value_cstring = CString::new(search_value)?;
        errno!(unsafe {
            libblkid_rs_sys::blkid_dev_set_search(
                self.0,
                search_type_cstring.as_ptr(),
                search_value_cstring.as_ptr(),
            )
        })?;
        Ok(self)
    }
}

impl Iterator for BlkidDevIter {
    type Item = BlkidDev;

    fn next(&mut self) -> Option<Self::Item> {
        let mut dev: libblkid_rs_sys::blkid_dev = ptr::null_mut();
        if unsafe { libblkid_rs_sys::blkid_dev_next(self.0, &mut dev as *mut _) } < 0 {
            None
        } else {
            assert!(!dev.is_null());
            Some(BlkidDev(dev))
        }
    }
}

impl Drop for BlkidDevIter {
    fn drop(&mut self) {
        unsafe { libblkid_rs_sys::blkid_dev_iterate_end(self.0) }
    }
}
