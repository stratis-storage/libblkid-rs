// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{ffi::CString, ptr};

use crate::{
    consts::{BlkidProbreqFlags, BlkidUsageFlags},
    err::BlkidErr,
    probe::BlkidProbe,
    Result,
};

impl BlkidProbe {
    /// Deprecated version of `blkid_probe_filter_superblocks_usage`.
    pub fn filter_usage(&mut self, flags: BlkidProbreqFlags, usage: BlkidUsageFlags) -> Result<()> {
        errno!(unsafe {
            libblkid_rs_sys::blkid_probe_filter_usage(self.0, flags.into(), usage.into())
        })
    }

    /// Deprecated version of `blkid_probe_filter_superblocks_type`
    pub fn filter_types(&mut self, flags: BlkidProbreqFlags, names: &[&str]) -> Result<()> {
        let cstring_vec: Vec<_> = names.iter().map(|name| CString::new(*name)).collect();
        if cstring_vec
            .iter()
            .any(|cstring_result| cstring_result.is_err())
        {
            return Err(BlkidErr::InvalidConv);
        }
        let checked_cstring_vec: Vec<_> =
            cstring_vec.into_iter().filter_map(|cs| cs.ok()).collect();
        let mut ptr_vec: Vec<_> = checked_cstring_vec
            .iter()
            .map(|cstring| cstring.as_ptr() as *mut _)
            .collect();
        ptr_vec.push(ptr::null_mut());

        errno!(unsafe {
            libblkid_rs_sys::blkid_probe_filter_types(self.0, flags.into(), ptr_vec.as_mut_ptr())
        })
    }

    /// Deprecated version of `blkid_probe_invert_superblocks_filter`.
    pub fn invert_filter(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_invert_filter(self.0) })
    }

    /// Deprecated version of `blkid_probe_reset_superblocks_filter`.
    pub fn reset_filter(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_reset_filter(self.0) })
    }
}
