use std::{ffi::CString, path::Path};

use libblkid_rs_sys::blkid_cache;

use crate::{
    consts::BlkidDevFlags,
    dev::{BlkidDev, BlkidDevIter},
    err::{BlkidErr, Result},
};

/// Data structure representing cache in libblkid
pub struct BlkidCache(blkid_cache);

impl BlkidCache {
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
}
