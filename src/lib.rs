#[macro_use]
mod macros;

mod cache;
mod dev;
mod err;

pub use crate::{
    cache::BlkidCache,
    dev::{BlkidDev, BlkidDevIter},
    err::{BlkidErr, Result},
};
