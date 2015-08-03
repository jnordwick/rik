#![allow(deprecated)]
#![feature(buf_stream)]

mod kobjects;
mod konnection;

use kobjects::KObject;
use kobjects::KObject::*;
use konnection::*;

// TODO: endian issues
//         byteorder (0.3.11)
//         - Library for reading/writing numbers in big-endian and little-endian.
//         -> https://crates.io/crates/byteorder

// TODO: Fix the char issues. K is bytes, Rust needs proper UTF-8.

fn main() {

    let mut kk = Konnection::konnect("localhost:5001", "abc", "").unwrap();
    println!("kk = {:?}", kk);

    let qq = kk.query("syms[1]");
    println!("qq = {:?}", qq);

    let rr = kk.response();
    println!("rr = {:?}", rr);

    println!("done");
}
