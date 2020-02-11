use std::{
    ffi::{CStr, CString},
    ptr,
};

use libc::c_char;

use crate::{err::BlkidErr, Result};

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
