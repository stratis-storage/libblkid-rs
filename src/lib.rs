// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # `libblkid-rs`
//!
//! `libblkid_rs` provides programmatic access in Rust to the C library
//! `libblkid`.
//!
//! ### Design
//!
//! The organization of the modules reflects the organization of the modules in the
//! C library. The main goal of this library is to maintain the same general
//! structure while taking advantage of Rust idioms.
//!
//! ### List of methods modified
//! * `blkid_devno_to_wholedisk` - This bindings method handles the buffer internally
//!   and therefore does not require a buffer argument. The limit for the maximum
//!   size of the returned device name is 4096 bytes. Please open an issue if more
//!   characters are required.
//! * `blkid_get_dev_size` - This method takes a `&Path` in the bindings
//!   and provides libblkid with the desired file descriptor.

#![deny(missing_docs)]

#[macro_use]
mod macros;

mod cache;
/// Module containing all typed constants
pub mod consts;
#[cfg(feature = "deprecated")]
mod deprecated;
mod dev;
mod devno;
mod encode;
mod err;
mod partition;
mod probe;
mod tag;
mod topology;
mod utils;
mod version;

pub use uuid::Uuid;

pub use libblkid_rs_sys::blkid_loff_t;

pub use crate::{
    cache::BlkidCache,
    consts::*,
    dev::{BlkidDev, BlkidDevIter},
    devno::{maj_t, min_t, BlkidDevno},
    encode::{encode_string, safe_string},
    err::{BlkidErr, Result},
    partition::{BlkidPartition, BlkidPartlist, BlkidParttable},
    probe::{
        get_partition_name, get_superblock_name, is_known_fs_type, is_known_partition_type,
        BlkidProbe,
    },
    tag::{parse_tag_string, BlkidTagIter},
    topology::BlkidTopology,
    utils::{evaluate_spec, evaluate_tag, send_uevent, BlkidSectors},
    version::{get_library_version, parse_version_string},
};
