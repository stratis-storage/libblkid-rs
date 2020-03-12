use std::{
    ffi::{CStr, CString},
    path::PathBuf,
    ptr,
};

use either::Either;
use libc::c_char;

use crate::{cache::BlkidCache, err::BlkidErr, Result};

/// Iterator for tags associated with a device
pub struct BlkidTagIter(libblkid_rs_sys::blkid_tag_iterate);

impl BlkidTagIter {
    pub(crate) fn new(iter: libblkid_rs_sys::blkid_tag_iterate) -> Self {
        BlkidTagIter(iter)
    }
}

impl Iterator for BlkidTagIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut type_: *const c_char = ptr::null_mut();
        let mut value: *const c_char = ptr::null_mut();
        if unsafe {
            libblkid_rs_sys::blkid_tag_next(
                self.0,
                &mut type_ as *mut *const _,
                &mut value as *mut *const _,
            )
        } < 0
        {
            None
        } else {
            assert!(!type_.is_null() && !value.is_null());
            let type_str = unsafe { CStr::from_ptr(type_) };
            let value_str = unsafe { CStr::from_ptr(value) };
            Some((
                type_str.to_str().ok()?.to_string(),
                value_str.to_str().ok()?.to_string(),
            ))
        }
    }
}

impl Drop for BlkidTagIter {
    fn drop(&mut self) {
        unsafe { libblkid_rs_sys::blkid_tag_iterate_end(self.0) }
    }
}

/// Parse a tag string into a tuple of type and value
pub fn parse_tag_string(tag_string: &str) -> Result<(String, String)> {
    let tag_cstring = CString::new(tag_string)?;
    let mut type_: *mut c_char = ptr::null_mut();
    let mut value: *mut c_char = ptr::null_mut();
    if unsafe {
        libblkid_rs_sys::blkid_parse_tag_string(
            tag_cstring.as_ptr(),
            &mut type_ as *mut *mut _,
            &mut value as *mut *mut _,
        )
    } < 0
    {
        Err(BlkidErr::LibErr)
    } else {
        assert!(!type_.is_null() && !value.is_null());
        let type_str = unsafe { CStr::from_ptr(type_) };
        let value_str = unsafe { CStr::from_ptr(value) };
        Ok((
            type_str.to_str()?.to_string(),
            value_str.to_str()?.to_string(),
        ))
    }
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
