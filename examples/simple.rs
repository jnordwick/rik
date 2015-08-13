#[macro_use]
extern crate rik;

use std::collections::hash_map::HashMap;

// TODO: endian issues
//         byteorder (0.3.11)
//         - Library for reading/writing numbers in big-endian and little-endian.
//         -> https://crates.io/crates/byteorder

// TODO: Fix the char issues. K is bytes, Rust needs proper UTF-8.

fn main() {

    let mut kk = rik::Konnection::konnect("localhost:5001", "abc", "").unwrap();
    println!("kk = {:?}", kk);

    let qq = kk.query("dd");
    println!("qq = {:?}", qq);

    let buf = kk.read_message();
    let (rr, ss) = rik::KObject::parse(buf);
    println!("rr={:?} ss={:?}", rr, ss);

    let mm = kdict_to_hashmap!(rik::KVector::Symbol, rik::KVector::Long, rr);
    println!("hashmap = {:?}", mm);

    println!("done");
}
