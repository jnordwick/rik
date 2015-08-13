#![allow(deprecated)]
#![feature(buf_stream)]
#![feature(convert)]

// TODO: endian issues
//         byteorder (0.3.11)
//         - Library for reading/writing numbers in big-endian and little-endian.
//         -> https://crates.io/crates/byteorder

// TODO: Fix the char issues. K is bytes, Rust needs proper UTF-8.

// TODO: tables, dicts
// TODO: remove header structs?
// TODO: maps to standard rust collections
// TODO: serialize to kdb
// TODO: functions
// TODO: collapse all atom/vector out, use guards on type codes instead?


pub mod konnection;
pub mod kobjects;

pub use konnection::*;
pub use kobjects::*;
