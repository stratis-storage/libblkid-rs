// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::{CStr, CString},
    path::Path,
};

use either::Either;

use libblkid_rs_sys::blkid_cache;

use crate::{
    consts::BlkidDevFlags,
    dev::{BlkidDev, BlkidDevIter},
    err::{BlkidErr, Result},
};

/// Data structure representing cache in libblkid
pub struct BlkidCache(blkid_cache);

impl BlkidCache {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut blkid_cache {
        &mut self.0 as *mut _
    }

    /// Save changes to cache file
    pub fn put_cache(&mut self) {
        unsafe { libblkid_rs_sys::blkid_put_cache(self.0) }
    }

    /// Allocate and initalize cache handler
    pub fn get_cache(&mut self, filename: &Path) -> Result<()> {
        let filename_cstring = CString::new(filename.to_str().ok_or(BlkidErr::InvalidConv)?)?;
        errno!(unsafe {
            libblkid_rs_sys::blkid_get_cache(&mut self.0 as *mut _, filename_cstring.as_ptr())
        })
    }

    /// Removes non-existant devices from cache
    pub fn gc_cache(&mut self) {
        unsafe { libblkid_rs_sys::blkid_gc_cache(self.0) }
    }

    /// Create an iterator from the cached devices
    pub fn iter(&self) -> BlkidDevIter {
        BlkidDevIter::new(unsafe { libblkid_rs_sys::blkid_dev_iterate_begin(self.0) })
    }

    /// Probe for all block devices
    pub fn probe_all(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_all(self.0) })
    }

    /// Probe for all new block devices
    pub fn probe_all_new(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_all_new(self.0) })
    }

    /// Probe for all removable block devices
    pub fn probe_all_removable(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_all_removable(self.0) })
    }

    /// Find a device by device name in the cache.
    ///
    /// Use the `BlkidDevConst::Create` flag to create an empty cache entry.
    pub fn get_dev(&self, devname: &str, flags: BlkidDevFlags) -> Result<BlkidDev> {
        let devname_cstring = CString::new(devname.as_bytes())?;
        Ok(BlkidDev::new(unsafe {
            libblkid_rs_sys::blkid_get_dev(self.0, devname_cstring.as_ptr(), flags.into())
        }))
    }

    /// Get the value associated with a tag (e.g. TYPE) for a given device
    pub fn get_tag_value(&self, tag_name: &str, devname: &str) -> Result<&str> {
        let tag_name_cstring = CString::new(tag_name.as_bytes())?;
        let devname_cstring = CString::new(devname.as_bytes())?;
        let ptr = errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_get_tag_value(
                self.0,
                tag_name_cstring.as_ptr(),
                devname_cstring.as_ptr(),
            )
        })?;
        Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?)
    }

    /// Get the device name for a specific `NAME=value` tag pair in the cache
    pub fn get_devname(&self, token_or_pair: Either<&str, (&str, &str)>) -> Result<&str> {
        let (name, value) = match token_or_pair {
            Either::Left(token) => {
                if !token.contains('=') {
                    return Err(BlkidErr::Other(
                        "Token input requires the format NAME=value".to_string(),
                    ));
                }
                let mut split = token.split('=');
                match (split.next(), split.next()) {
                    (Some(name), Some(value)) => (name, value),
                    (_, _) => {
                        return Err(BlkidErr::Other(
                            "Token input requires the format NAME=value".to_string(),
                        ));
                    }
                }
            }
            Either::Right((name, value)) => (name, value),
        };
        let name_cstring = CString::new(name.as_bytes())?;
        let value_cstring = CString::new(value.as_bytes())?;
        let ptr = errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_get_devname(
                self.0,
                name_cstring.as_ptr(),
                value_cstring.as_ptr(),
            )
        })?;
        Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?)
    }

    /// Find the device with the specified tag
    pub fn find_dev_with_tag(&self, type_: &str, value: &str) -> Result<BlkidDev> {
        let type_cstring = CString::new(type_)?;
        let value_cstring = CString::new(value)?;
        let ptr = errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_find_dev_with_tag(
                self.0,
                type_cstring.as_ptr(),
                value_cstring.as_ptr(),
            )
        })?;
        Ok(BlkidDev::new(ptr))
    }
}
