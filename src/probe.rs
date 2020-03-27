use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    os::unix::io::RawFd,
    path::Path,
    ptr,
};

use crate::{
    consts::{BlkidFltr, BlkidSublksFlags, BlkidUsageFlags},
    devno::BlkidDevno,
    err::BlkidErr,
    Result,
};

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

    /// Get the device number associated with the probe device.
    pub fn get_devno(&self) -> BlkidDevno {
        BlkidDevno::new(unsafe { libblkid_rs_sys::blkid_probe_get_devno(self.0) })
    }

    /// Get the device number of the whole disk
    pub fn get_wholedisk_devno(&self) -> BlkidDevno {
        BlkidDevno::new(unsafe { libblkid_rs_sys::blkid_probe_get_wholedisk_devno(self.0) })
    }

    /// Check if the given device is an entire disk (instead of a partition or
    /// something similar)
    pub fn is_wholedisk(&self) -> bool {
        (unsafe { libblkid_rs_sys::blkid_probe_is_wholedisk(self.0) }) > 0
    }

    /// Get the size of of a device.
    pub fn get_size(&self) -> libblkid_rs_sys::blkid_loff_t {
        unsafe { libblkid_rs_sys::blkid_probe_get_size(self.0) }
    }

    /// Get the offset of a probing area of a device.
    pub fn get_offset(&self) -> libblkid_rs_sys::blkid_loff_t {
        unsafe { libblkid_rs_sys::blkid_probe_get_offset(self.0) }
    }

    /// Get the sector size of the attached device.
    pub fn get_sector_size(&self) -> libc::c_uint {
        unsafe { libblkid_rs_sys::blkid_probe_get_sectorsize(self.0) }
    }

    /// Get a file descriptor associated with the given device.
    pub fn get_fd(&self) -> Result<RawFd> {
        errno_with_ret!(unsafe { libblkid_rs_sys::blkid_probe_get_fd(self.0) })
    }

    /// Enable superblock probing.
    pub fn enable_superblocks(&mut self, enable: bool) -> Result<()> {
        errno!(unsafe {
            libblkid_rs_sys::blkid_probe_enable_superblocks(self.0, if enable { 1 } else { 0 })
        })
    }

    /// Set the superblock probing flags.
    pub fn set_superblock_flags(&mut self, flags: BlkidSublksFlags) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_set_superblocks_flags(self.0, flags.into()) })
    }

    /// Reset the superblock probing filter.
    pub fn reset_superblock_filter(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_reset_superblocks_filter(self.0) })
    }

    /// Invert the superblock probing filter.
    pub fn invert_superblock_filter(&mut self) -> Result<()> {
        errno!(unsafe { libblkid_rs_sys::blkid_probe_invert_superblocks_filter(self.0) })
    }

    /// Filter superblock types based on the provided flags and name.
    pub fn filter_superblock_type(&mut self, flag: BlkidFltr, names: &[&str]) -> Result<()> {
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
            libblkid_rs_sys::blkid_probe_filter_superblocks_type(
                self.0,
                flag.into(),
                ptr_vec.as_mut_ptr(),
            )
        })
    }
}

impl Drop for BlkidProbe {
    fn drop(&mut self) {
        unsafe { libblkid_rs_sys::blkid_free_probe(self.0) }
    }
}

/// Check if the given string containing a filesystem name is a known filesystem
/// type.
pub fn is_known_fs_type(fstype: &str) -> Result<bool> {
    let fstype_cstring = CString::new(fstype)?;
    Ok(unsafe { libblkid_rs_sys::blkid_known_fstype(fstype_cstring.as_ptr()) } > 0)
}

/// Get the name and flags of a superblock at the given index in the libblkid
/// internal state.
///
/// This method in libblkid exposes implementation details of the library. There
/// is no way to map indicies to types without duplicating logic inside and outside
/// of the library.
pub fn get_superblock_name(
    index: usize,
    get_name: bool,
    get_flags: bool,
) -> Result<(Option<&'static str>, Option<BlkidUsageFlags>)> {
    let mut name_ptr: *const libc::c_char = ptr::null();
    let mut flags: libc::c_int = 0;
    errno!(unsafe {
        libblkid_rs_sys::blkid_superblocks_get_name(
            index,
            if get_name {
                &mut name_ptr as *mut _
            } else {
                ptr::null_mut()
            },
            if get_flags {
                &mut flags as *mut _
            } else {
                ptr::null_mut()
            },
        )
    })?;
    let name_option = Some(unsafe { CStr::from_ptr(name_ptr) }.to_str()?);
    let flags_option = Some(BlkidUsageFlags::try_from(flags)?);
    Ok((name_option, flags_option))
}
