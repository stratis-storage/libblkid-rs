use std::{ffi::CStr, ptr};

use libc::c_char;

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
