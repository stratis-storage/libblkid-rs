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

#![deny(missing_docs)]

#[macro_use]
mod macros;

mod cache;
/// Module containing all typed constants
pub mod consts;
mod dev;
mod devno;
mod err;

pub use crate::{
    cache::BlkidCache,
    dev::{BlkidDev, BlkidDevIter},
    devno::BlkidDevno,
    err::{BlkidErr, Result},
};
