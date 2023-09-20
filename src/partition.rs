// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi::CStr;

use uuid::Uuid;

use crate::{
    devno::BlkidDevno,
    err::BlkidErr,
    utils::{BlkidBytes, BlkidSectors},
    Result,
};

/// A handle for working with partition tables.
pub struct BlkidParttable(libblkid_rs_sys::blkid_parttable);

impl BlkidParttable {
    /// Get the type of the partition table.
    pub fn get_type(&self) -> Result<String> {
        Ok(unsafe {
            CStr::from_ptr(errno_ptr!(libblkid_rs_sys::blkid_parttable_get_type(
                self.0
            ))?)
        }
        .to_str()?
        .to_string())
    }

    /// Get the ID of the partition table. Given that this could be a UUID or some
    /// other form of identifier, the return value is a `String` to cover all cases.
    pub fn get_id(&self) -> Result<String> {
        Ok(
            unsafe { CStr::from_ptr(errno_ptr!(libblkid_rs_sys::blkid_parttable_get_id(self.0))?) }
                .to_str()?
                .to_string(),
        )
    }

    /// Get the offset of the partition table in bytes.
    pub fn get_offset(&self) -> Result<BlkidBytes> {
        Ok(BlkidBytes::new(errno_with_ret!(unsafe {
            libblkid_rs_sys::blkid_parttable_get_offset(self.0)
        })?))
    }

    /// Get the parent partition in the case of nested partition tables.
    pub fn get_parent(&self) -> Option<BlkidPartition> {
        option_ptr!(unsafe { libblkid_rs_sys::blkid_parttable_get_parent(self.0) })
            .map(BlkidPartition)
    }
}

/// A handle for working with a probed partition.
pub struct BlkidPartition(libblkid_rs_sys::blkid_partition);

impl BlkidPartition {
    /// Get the partition table for the given partition.
    pub fn get_table(&self) -> Result<BlkidParttable> {
        Ok(BlkidParttable(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_partition_get_table(self.0)
        })?))
    }

    /// Get the partition name or `None` if it can't be represented.
    pub fn get_name(&self) -> Result<Option<String>> {
        let char_ptr =
            match option_ptr!(unsafe { libblkid_rs_sys::blkid_partition_get_name(self.0) }) {
                Some(ptr) => ptr,
                None => return Ok(None),
            };
        unsafe { CStr::from_ptr(char_ptr) }
            .to_str()
            .map(|s| Some(s.to_string()))
            .map_err(BlkidErr::UTF8)
    }

    /// Get the partition UUID or `None` if the partition table doesn't support it.
    pub fn get_uuid(&self) -> Result<Option<Uuid>> {
        let char_ptr =
            match option_ptr!(unsafe { libblkid_rs_sys::blkid_partition_get_uuid(self.0) }) {
                Some(ptr) => ptr,
                None => return Ok(None),
            };
        match unsafe { CStr::from_ptr(char_ptr) }.to_str() {
            Ok(s) => Ok(Some(Uuid::parse_str(s).map_err(BlkidErr::Uuid)?)),
            Err(e) => Err(BlkidErr::UTF8(e)),
        }
    }

    /// Get the partition number.
    pub fn get_partno(&self) -> Result<libc::c_uint> {
        errno_with_ret!(unsafe { libblkid_rs_sys::blkid_partition_get_partno(self.0) })
            .map(|i| i as libc::c_uint)
    }

    /// Get the start of the partition in units of sectors.
    pub fn get_start(&self) -> BlkidSectors {
        BlkidSectors::new(unsafe { libblkid_rs_sys::blkid_partition_get_start(self.0) })
    }

    /// Get the size of the partition in units of sectors.
    pub fn get_size(&self) -> BlkidSectors {
        BlkidSectors::new(unsafe { libblkid_rs_sys::blkid_partition_get_size(self.0) })
    }

    /// Get the numeric partition type. Use `get_type_string` for the `String`
    /// representation.
    pub fn get_type(&self) -> libc::c_int {
        unsafe { libblkid_rs_sys::blkid_partition_get_type(self.0) }
    }

    /// Get the string representation of the partition type.
    pub fn get_type_string(&self) -> Result<String> {
        Ok(unsafe {
            CStr::from_ptr(errno_ptr!(
                libblkid_rs_sys::blkid_partition_get_type_string(self.0)
            )?)
        }
        .to_str()?
        .to_string())
    }

    /// Get the flags for the given partition.
    ///
    /// This method is not typed as the documentation does not specify which
    /// constants are used as flags.
    pub fn get_flags(&self) -> libc::c_ulonglong {
        unsafe { libblkid_rs_sys::blkid_partition_get_flags(self.0) }
    }

    /// Check whether the given partition is logical.
    pub fn is_logical(&self) -> bool {
        (unsafe { libblkid_rs_sys::blkid_partition_is_logical(self.0) }) != 0
    }

    /// Check whether the given partition is an extended partition.
    pub fn is_extended(&self) -> bool {
        (unsafe { libblkid_rs_sys::blkid_partition_is_extended(self.0) }) != 0
    }

    /// Check whether the given partition is a primary partition.
    pub fn is_primary(&self) -> bool {
        (unsafe { libblkid_rs_sys::blkid_partition_is_primary(self.0) }) != 0
    }
}

/// A handle for traversing a list of partitions.
pub struct BlkidPartlist(libblkid_rs_sys::blkid_partlist);

impl BlkidPartlist {
    pub(crate) fn new(partlist: libblkid_rs_sys::blkid_partlist) -> BlkidPartlist {
        BlkidPartlist(partlist)
    }

    /// Get the number of partitions in the list.
    pub fn number_of_partitions(&mut self) -> Result<libc::c_int> {
        errno_with_ret!(unsafe { libblkid_rs_sys::blkid_partlist_numof_partitions(self.0) })
    }

    /// Get the partition table for a list of partitions.
    pub fn get_table(&mut self) -> Result<BlkidParttable> {
        Ok(BlkidParttable(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_partlist_get_table(self.0)
        })?))
    }

    /// Get a partition at the given index of the list.
    pub fn get_partition(&mut self, index: libc::c_int) -> Result<BlkidPartition> {
        Ok(BlkidPartition(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_partlist_get_partition(self.0, index)
        })?))
    }

    /// Get a partition by the number listed in the partition table. Correctly
    /// handles "out-of-order" partition tables.
    pub fn get_partition_by_partno(&mut self, num: libc::c_int) -> Result<BlkidPartition> {
        Ok(BlkidPartition(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_partlist_get_partition_by_partno(self.0, num)
        })?))
    }

    /// Get a partition using the device number of a partition.
    pub fn get_partition_by_devno(&mut self, dev: &BlkidDevno) -> Result<BlkidPartition> {
        Ok(BlkidPartition(errno_ptr!(unsafe {
            libblkid_rs_sys::blkid_partlist_devno_to_partition(self.0, dev.as_dev_t())
        })?))
    }
}
