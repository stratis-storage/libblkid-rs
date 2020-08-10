// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::{CStr, CString},
    path::{Path, PathBuf},
    ptr,
};

use either::Either;

use crate::{cache::BlkidCache, err::BlkidErr, Result};

const SECTOR_SIZE: libblkid_rs_sys::blkid_loff_t = 512;

/// A struct representing a count with units of sectors.
pub struct BlkidSectors(libblkid_rs_sys::blkid_loff_t);

impl BlkidSectors {
    pub(crate) fn new(num: libblkid_rs_sys::blkid_loff_t) -> Self {
        BlkidSectors(num)
    }

    /// Return the number of bytes represented by this number of disk sectors.
    pub fn bytes(&self) -> BlkidBytes {
        BlkidBytes(self.0 * SECTOR_SIZE)
    }
}

impl AsRef<libblkid_rs_sys::blkid_loff_t> for BlkidSectors {
    fn as_ref(&self) -> &libblkid_rs_sys::blkid_loff_t {
        &self.0
    }
}

/// A struct repsenting a count with units of bytes.
pub struct BlkidBytes(libblkid_rs_sys::blkid_loff_t);

impl BlkidBytes {
    pub(crate) fn new(num: libblkid_rs_sys::blkid_loff_t) -> Self {
        BlkidBytes(num)
    }

    /// Return the number of sectors repesented by this number of bytes or
    /// an error if the number of bytes is not divisible by the sector size.
    pub fn sectors(&self) -> Result<BlkidSectors> {
        if self.0 % SECTOR_SIZE != 0 {
            Err(BlkidErr::InvalidConv)
        } else {
            Ok(BlkidSectors(self.0 / SECTOR_SIZE))
        }
    }
}

impl AsRef<libblkid_rs_sys::blkid_loff_t> for BlkidBytes {
    fn as_ref(&self) -> &libblkid_rs_sys::blkid_loff_t {
        &self.0
    }
}

/// Send a uevent to a device specified by the device path
pub fn send_uevent(dev: &Path, action: &str) -> Result<()> {
    let dev_cstring = CString::new(dev.display().to_string())?;
    let action_cstring = CString::new(action)?;
    errno!(unsafe {
        libblkid_rs_sys::blkid_send_uevent(dev_cstring.as_ptr(), action_cstring.as_ptr())
    })
}

fn evaluate(
    tag_or_tuple: Either<(&str, &str), &str>,
    cache: Option<&mut BlkidCache>,
) -> Result<PathBuf> {
    let cache_ptr = match cache {
        Some(c) => c.as_mut_ptr(),
        None => ptr::null_mut(),
    };
    let allocated_string = match tag_or_tuple {
        Either::Left((token, value)) => {
            let token_cstring = CString::new(token)?;
            let value_cstring = CString::new(value)?;
            errno_ptr!(unsafe {
                libblkid_rs_sys::blkid_evaluate_tag(
                    token_cstring.as_ptr(),
                    value_cstring.as_ptr(),
                    cache_ptr,
                )
            })?
        }
        Either::Right(spec) => {
            let spec_cstring = CString::new(spec)?;
            errno_ptr!(unsafe {
                libblkid_rs_sys::blkid_evaluate_spec(spec_cstring.as_ptr(), cache_ptr)
            })?
        }
    };
    let rust_cstr = unsafe { CStr::from_ptr(allocated_string) };
    let return_string = rust_cstr.to_str()?.to_string();
    unsafe { libc::free(allocated_string as *mut libc::c_void) };
    Ok(PathBuf::from(return_string))
}

/// Find the path of a device matching a tag
pub fn evaluate_tag(token: &str, value: &str, cache: Option<&mut BlkidCache>) -> Result<PathBuf> {
    evaluate(Either::Left((token, value)), cache)
}

/// Find the path of a device matching an unparsed tag or a path to a device mapper
/// node such as `/dev/dm-0`
pub fn evaluate_spec(tag_or_dm_path: &str, cache: Option<&mut BlkidCache>) -> Result<PathBuf> {
    evaluate(Either::Right(tag_or_dm_path), cache)
}
