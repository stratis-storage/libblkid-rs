use crate::Result;

use crate::devno::BlkidDevno;

/// A handle for working with partition tables.
pub struct BlkidParttable(libblkid_rs_sys::blkid_parttable);

/// A handle for working with a probed partition.
pub struct BlkidPartition(libblkid_rs_sys::blkid_partition);

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
