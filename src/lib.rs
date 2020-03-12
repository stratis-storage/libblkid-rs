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
//!
//! ### List of methods not included
//! * `blkid_verify` - This method is not included because the struct flag
//!   that will notify the caller of whether it succeeded or not cannot be accessed
//!   from the public API.

#![deny(missing_docs)]

#[macro_use]
mod macros;

mod cache;
/// Module containing all typed constants
pub mod consts;
mod dev;
mod devno;
mod encode;
mod err;
mod tag;
mod utils;
mod version;

pub use crate::{
    cache::BlkidCache,
    dev::{BlkidDev, BlkidDevIter},
    devno::BlkidDevno,
    encode::{encode_string, safe_string},
    err::{BlkidErr, Result},
    tag::{evaluate_spec, evaluate_tag, parse_tag_string, BlkidTagIter},
    utils::send_uevent,
    version::{get_library_version, parse_version_string},
};
