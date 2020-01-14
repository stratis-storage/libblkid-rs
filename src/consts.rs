use libc::c_int;

consts_enum_conv!(
    /// Flags for blockdev cache operations
    BlkidDevFlag <=> c_int,
    #[allow(missing_docs)]
    Find => libblkid_rs_sys::BLKID_DEV_FIND as c_int,
    #[allow(missing_docs)]
    Create => libblkid_rs_sys::BLKID_DEV_CREATE as c_int,
    #[allow(missing_docs)]
    Verify => libblkid_rs_sys::BLKID_DEV_VERIFY as c_int,
    #[allow(missing_docs)]
    Normal => libblkid_rs_sys::BLKID_DEV_NORMAL as c_int
);

flags!(
    /// Set of flags of type `BlkidDevFlag`
    BlkidDevFlags <=> c_int,
    BlkidDevFlag
);
