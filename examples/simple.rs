#![feature(convert)]

extern crate rik;

// TODO: endian issues
//         byteorder (0.3.11)
//         - Library for reading/writing numbers in big-endian and little-endian.
//         -> https://crates.io/crates/byteorder

// TODO: Fix the char issues. K is bytes, Rust needs proper UTF-8.

fn main() {

    let mut kk = rik::Konnection::konnect("localhost:5001", "abc", "").unwrap();
    println!("kk = {:?}", kk);

    let qq = kk.query("randoms");
    println!("qq = {:?}", qq);

    kk.read_message();
    let (rr, ss) = rik::KObject::parse(kk.buf.as_slice());
    println!("ss={:?} rr = {:?}", rr, ss);

    println!("done");
}
