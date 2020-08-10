// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    ffi::{CStr, CString},
    ptr,
};

use libc::{c_char, c_int};

use crate::Result;

/// Parse a version string into a version code
pub fn parse_version_string(version_str: &str) -> Result<c_int> {
    let version_cstring = CString::new(version_str)?;
    Ok(unsafe { libblkid_rs_sys::blkid_parse_version_string(version_cstring.as_ptr()) })
}

/// Get library version
pub fn get_library_version() -> Result<(c_int, String, String)> {
    let mut ver_ptr: *const c_char = ptr::null();
    let mut date_ptr: *const c_char = ptr::null();
    let ver_code = unsafe {
        libblkid_rs_sys::blkid_get_library_version(
            &mut ver_ptr as *mut *const _,
            &mut date_ptr as *mut *const _,
        )
    };
    let ver_string = unsafe { CStr::from_ptr(ver_ptr) }.to_str()?.to_string();
    let date_string = unsafe { CStr::from_ptr(date_ptr) }.to_str()?.to_string();
    Ok((ver_code, ver_string, date_string))
}
