/// A handle for traversing the blkid topology of devices.
pub struct BlkidTopology(libblkid_rs_sys::blkid_topology);

impl BlkidTopology {
    pub(crate) fn new(topology: libblkid_rs_sys::blkid_topology) -> BlkidTopology {
        BlkidTopology(topology)
    }

    /// Get the alignment offset.
    pub fn get_alignment_offset(&self) -> libc::c_ulong {
        unsafe { libblkid_rs_sys::blkid_topology_get_alignment_offset(self.0) }
    }

    /// Get the minimum size of an IO operation.
    pub fn get_minimum_io_size(&self) -> libc::c_ulong {
        unsafe { libblkid_rs_sys::blkid_topology_get_minimum_io_size(self.0) }
    }

    /// Get the optimal size of an IO operation.
    pub fn get_optimal_io_size(&self) -> libc::c_ulong {
        unsafe { libblkid_rs_sys::blkid_topology_get_optimal_io_size(self.0) }
    }

    /// Get the size of a logical sector.
    pub fn get_logical_sector_size(&self) -> libc::c_ulong {
        unsafe { libblkid_rs_sys::blkid_topology_get_logical_sector_size(self.0) }
    }

    /// Get the size of a physical sector.
    pub fn get_physical_sector_size(&self) -> libc::c_ulong {
        unsafe { libblkid_rs_sys::blkid_topology_get_physical_sector_size(self.0) }
    }
}
