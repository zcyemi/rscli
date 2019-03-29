#![allow(dead_code)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(core_intrinsics)]
#![feature(untagged_unions)]

#[macro_use]
extern crate lazy_static;

pub mod il;
pub mod util;
pub mod data;
pub mod context;
pub mod loader;
pub mod meta;
pub mod tbl;
pub mod reader;
pub mod reflection;
pub mod winpe;

#[cfg(test)]
pub mod test;