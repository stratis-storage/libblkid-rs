// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

consts_enum_conv!(
    /// Flags for blockdev usage
    BlkidUsageFlag <=> c_int,
    #[allow(missing_docs)]
    Filesystem => libblkid_rs_sys::BLKID_USAGE_FILESYSTEM as c_int,
    #[allow(missing_docs)]
    Raid => libblkid_rs_sys::BLKID_USAGE_RAID as c_int,
    #[allow(missing_docs)]
    Crypto => libblkid_rs_sys::BLKID_USAGE_CRYPTO as c_int,
    #[allow(missing_docs)]
    Other => libblkid_rs_sys::BLKID_USAGE_OTHER as c_int
);

flags!(
    /// Set of flags of type `BlkidDevFlag`
    BlkidUsageFlags <=> c_int,
    BlkidUsageFlag
);

consts_enum_conv!(
    /// Flags for superblock probing
    BlkidSublks <=> c_int,
    /// Read label from superblock
    Label => libblkid_rs_sys::BLKID_SUBLKS_LABEL as c_int,
    /// Read label from superblock and define `LABEL_RAW` value
    Labelraw => libblkid_rs_sys::BLKID_SUBLKS_LABELRAW as c_int,
    /// Read UUID from superblock
    Uuid => libblkid_rs_sys::BLKID_SUBLKS_UUID as c_int,
    /// Read UUID from superblock and define `UUID_RAW` value
    Uuidraw => libblkid_rs_sys::BLKID_SUBLKS_UUID as c_int,
    /// Read type from superblock and define `TYPE` value
    Type => libblkid_rs_sys::BLKID_SUBLKS_TYPE as c_int,
    /// Read compatible filesystem type from superblock
    Sectype => libblkid_rs_sys::BLKID_SUBLKS_SECTYPE as c_int,
    /// Read usage from superblock and define `USAGE` value
    Usage => libblkid_rs_sys::BLKID_SUBLKS_USAGE as c_int,
    /// Read filesystem version from superblock
    Version => libblkid_rs_sys::BLKID_SUBLKS_VERSION as c_int,
    /// Read superblock magic number and define `SBMAGIC` and `SBMAGIC_OFFSET`
    Magic => libblkid_rs_sys::BLKID_SUBLKS_MAGIC as c_int,
    /// Allow a bad checksum
    Badcsum => libblkid_rs_sys::BLKID_SUBLKS_BADCSUM as c_int,
    /// Default flags
    Default => libblkid_rs_sys::BLKID_SUBLKS_DEFAULT as c_int
);

flags!(
    /// Set of `BlkidSublks` flags
    BlkidSublksFlags <=> c_int,
    BlkidSublks
);

consts_enum_conv!(
    /// Constants for probing superblock type.
    BlkidFltr <=> c_int,
    /// Probe for all names that are not in the list that was provided.
    Notin => libblkid_rs_sys::BLKID_FLTR_NOTIN as c_int,
    /// Probe for all names that are in the list that was provided.
    Onlyin => libblkid_rs_sys::BLKID_FLTR_ONLYIN as c_int
);

consts_enum_conv!(
    /// Low level tag probing return value
    BlkidProbeRet <=> c_int,
    /// Successful probe
    Success => 0,
    /// Done probing
    Done => 1
);

consts_enum_conv!(
    /// Low level tag probing return value
    BlkidSafeprobeRet <=> c_int,
    /// Successful probe
    Success => 0,
    /// Nothing detected
    None => 1,
    /// Ambiguous result detected
    Ambiguous => -2
);

consts_enum_conv!(
    /// Low level tag probing return value
    BlkidFullprobeRet <=> c_int,
    /// Successful probe
    Success => 0,
    /// Nothing detected
    None => 1
);

consts_enum_conv!(
    /// Probe request flags
    BlkidProbreqFlag <=> c_int,
    #[allow(missing_docs)]
    Label => libblkid_rs_sys::BLKID_PROBREQ_LABEL as i32,
    #[allow(missing_docs)]
    Labelraw => libblkid_rs_sys::BLKID_PROBREQ_LABELRAW as i32,
    #[allow(missing_docs)]
    Uuid => libblkid_rs_sys::BLKID_PROBREQ_UUID as i32,
    #[allow(missing_docs)]
    Uuidraw => libblkid_rs_sys::BLKID_PROBREQ_UUIDRAW as i32,
    #[allow(missing_docs)]
    Type => libblkid_rs_sys::BLKID_PROBREQ_TYPE as i32,
    #[allow(missing_docs)]
    Sectype => libblkid_rs_sys::BLKID_PROBREQ_SECTYPE as i32,
    #[allow(missing_docs)]
    Usage => libblkid_rs_sys::BLKID_PROBREQ_USAGE as i32,
    #[allow(missing_docs)]
    Version => libblkid_rs_sys::BLKID_PROBREQ_VERSION as i32
);

flags!(
    /// Set of flags of type `BlkidProbreqFlag`
    BlkidProbreqFlags <=> c_int,
    BlkidProbreqFlag
);
