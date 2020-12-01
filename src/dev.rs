// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::{CStr, CString},
    fs::File,
    os::unix::io::AsRawFd,
    ptr,
};

use crate::{err::Result, tag::BlkidTagIter};

/// Size of a device as reported by libblkid
pub struct BlkidSize(libblkid_rs_sys::blkid_loff_t);

impl Into<i64> for BlkidSize {
    fn into(self) -> i64 {
        self.0
    }
}

/// Block device found by blkid
pub struct BlkidDev(libblkid_rs_sys::blkid_dev);

impl BlkidDev {
    pub(crate) fn new(inner: libblkid_rs_sys::blkid_dev) -> Self {
        BlkidDev(inner)
    }

    /// Get the device name for a blkid device
    pub fn devname(&self) -> Result<&str> {
        let ret = errno_ptr!(unsafe { libblkid_rs_sys::blkid_dev_devname(self.0) })?;
        let cstr_ret = unsafe { CStr::from_ptr(ret) };
        Ok(cstr_ret.to_str()?)
    }

    /// Get the size of a device as reported by the cache
    pub fn devsize(&self) -> Result<BlkidSize> {
        let f = File::open(self.devname()?)?;
        Ok(BlkidSize(unsafe {
            libblkid_rs_sys::blkid_get_dev_size(f.as_raw_fd())
        }))
    }

    /// Iterate through tags in associated with the given block device
    pub fn tag_iter(&self) -> BlkidTagIter {
        BlkidTagIter::new(unsafe { libblkid_rs_sys::blkid_tag_iterate_begin(self.0) })
    }

    /// Return `true` if the given device has a specified tag
    pub fn has_tag(&self, type_: &str, value: &str) -> Result<bool> {
        let type_cstring = CString::new(type_)?;
        let value_cstring = CString::new(value)?;
        Ok(unsafe {
            libblkid_rs_sys::blkid_dev_has_tag(
                self.0,
                type_cstring.as_ptr(),
                value_cstring.as_ptr(),
            )
        } != 0)
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
                search_type_cstring.as_ptr() as *mut _,
                search_value_cstring.as_ptr() as *mut _,
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
