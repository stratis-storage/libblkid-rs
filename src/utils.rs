use std::{ffi::CString, path::Path};

use crate::Result;

/// Send a uevent to a device specified by the device path
pub fn send_uevent(dev: &Path, action: &str) -> Result<()> {
    let dev_cstring = CString::new(dev.display().to_string())?;
    let action_cstring = CString::new(action)?;
    errno!(unsafe {
        libblkid_rs_sys::blkid_send_uevent(
            dev_cstring.as_ptr(),
            action_cstring.as_ptr(),
        )
    })
}
