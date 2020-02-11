use std::ffi::CString;

use libc::c_int;

use crate::Result;

/// Parse a version string into a version code
pub fn parse_version_string(version_str: &str) -> Result<c_int> {
    let version_cstring = CString::new(version_str)?;
    Ok(unsafe { libblkid_rs_sys::blkid_parse_version_string(version_cstring.as_ptr()) })
}
