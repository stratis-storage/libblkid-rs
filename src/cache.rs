use std::{
    ffi::CString,
    path::Path,
};

use libblkid_rs_sys::blkid_cache;

use crate::err::{BlkidErr, Result};

/// Data structure representing cache in libblkid
pub struct BlkidCache(blkid_cache);

impl BlkidCache {
    /// Save cahnges to cache file
    pub fn put_cache(&mut self) {
        unsafe { libblkid_rs_sys::blkid_put_cache(self.0) }
    }

    /// Allocate and initalize cache handler
    pub fn get_cache(&mut self, filename: &Path) -> Result<()> {
        let filename_cstring = CString::new(
            filename.to_str()
                .ok_or(BlkidErr::InvalidConv)?
        ).map_err(BlkidErr::Null)?;
        errno!(unsafe {
            libblkid_rs_sys::blkid_get_cache(&mut self.0 as *mut _, filename_cstring.as_ptr())
        })
    }

    /// Removes non-existant devices from cache
    pub fn gc_cache(&mut self) {
        unsafe {
            libblkid_rs_sys::blkid_gc_cache(self.0)
        }
    }
}
