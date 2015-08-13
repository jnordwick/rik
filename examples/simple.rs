#[macro_use]
extern crate rik;

use std::collections::hash_map::HashMap;

fn main() {

    // connect
    let mut kk = rik::Konnection::konnect("localhost:5001", "abc", "").unwrap();
    println!("kk = {:?}", kk);

    {
        // Make a dictionary.
        // Most K structures are returned as either the underlying,
        // a Vec<T>, or a struct of the appropriate Vecs.

        let qq = kk.query("`a`b`c!1 2 3");
        let buf = kk.read_message();
        let (rr, ss) = rik::KObject::parse(buf);
        println!("size={:?} read={:?}", ss, rr);

        // Some macro magic to get the right types out
        // rust kills you on the typechecking.
        // The dictionary is a (Vec<K>,Vec<V>) tuple struct,
        // and this extracts what you are expecting and puts it
        // into a HashMap for you.
        let mm = kdict_to_hashmap!(rik::KVector::Symbol, rik::KVector::Long, rr);
        println!("hashmap = {:?}", mm);
    }

    {   // Try a basic symbol
        kk.query("`$\"asd\"");
        let (rr, _) = rik::KObject::parse(kk.read_message());
        if let rik::KObject::Atom(rik::KAtom::Symbol(s)) = rr {
            println!("sym = {}", s);
        }
    }

    {   // And now a vector
        kk.query("1.1*key 10");
        let (rr, _) = rik::KObject::parse(kk.read_message());
        if let rik::KObject::Vector(rik::KVector::Float(f)) = rr {
            println!("float = {:?}", f);
        }
    }

    println!("done");
}
