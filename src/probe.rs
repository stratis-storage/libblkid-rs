use std::{ffi::CString, os::unix::io::RawFd, path::Path};

use crate::{err::BlkidErr, Result};

/// A structure for probing block devices.
pub struct BlkidProbe(libblkid_rs_sys::blkid_probe);

impl BlkidProbe {
    /// Allocate and create a new libblkid probe.
    pub fn new() -> Result<Self> {
        Ok(BlkidProbe(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_new_probe()
        })?))
    }

    /// Create a new probe from a filename.
    pub fn new_from_filename(filename: &Path) -> Result<Self> {
        let filename_cstring =
            CString::new(filename.to_str().ok_or_else(|| BlkidErr::InvalidConv)?)?;
        Ok(BlkidProbe(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_new_probe_from_filename(filename_cstring.as_ptr())
        })?))
    }

    /// Reset the probe.
    pub fn reset(&mut self) {
        unsafe { libblkid_rs_sys::blkid_reset_probe(self.0) }
    }

    /// Reset and free all buffers used in the probe.
    pub fn reset_buffers(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_reset_buffers(self.0) })
    }

    /// Hide a memory range in the probe from the next `do_probe` call.
    pub fn hide_range(&mut self, offset: u64, len: u64) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_hide_range(self.0, offset, len) })
    }

    /// Assign the device to the probe control structure.
    pub fn set_device(
        &mut self,
        fd: RawFd,
        offset: libblkid_rs_sys::blkid_loff_t,
        size: libblkid_rs_sys::blkid_loff_t,
    ) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_set_device(self.0, fd, offset, size) })
    }
}

impl Drop for BlkidProbe {
    fn drop(&mut self) {
        unsafe { libblkid_rs_sys::blkid_free_probe(self.0) }
    }
}
