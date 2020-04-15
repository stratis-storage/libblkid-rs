use std::ffi::CStr;

use uuid::Uuid;

use crate::{devno::BlkidDevno, err::BlkidErr, utils::BlkidSectors, Result};

/// A handle for working with partition tables.
pub struct BlkidParttable(libblkid_rs_sys::blkid_parttable);

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
