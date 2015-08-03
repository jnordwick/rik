#[allow(deprecated)]

use std::io::{self, BufStream};
use std::io::prelude::*;
use std::net::*;
use std::result::Result;
use std::slice;
use std::mem;

use kobjects::KObject;
use kobjects::KObject::*;

#[derive(Debug)]
pub struct Konnection {
    stream: BufStream<TcpStream>,
    hp: String,
    cred: String,
    ver: u8,
}

#[derive(Debug)]
#[repr(packed)]
struct KMessageHeader {
    endian: i8,
    msg_type: i8,
    compress: i8,
    unused: i8,
    size: i32,
}

#[derive(Debug)]
#[repr(packed)]
struct KVectorHeader {
    val_type: i8,
    attrib: i8,
    len: i32,
}

fn struct_to_bytes<T>(s: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts(s as *const _ as *const _, mem::size_of::<T>())
    }
}

fn struct_to_bytes_mut<T>(s: &mut T) -> &mut [u8] {
    unsafe {
        ::std::slice::from_raw_parts_mut(s as *mut _ as *mut _, mem::size_of::<T>())
    }
}

fn read_all(r: &mut BufStream<TcpStream>, buf: &mut [u8]) {
    let len = buf.len();
    let mut n = 0;
    while n < len {
        n += r.read(&mut buf[n..len]).unwrap();
    }
}

impl Konnection {

    pub fn konnect(hostport: &str, name: &str, passwd: &str)
                   -> io::Result<Konnection> {

        let hp = String::from(hostport);
        let sock = try!(TcpStream::connect(hostport));
        let cred = format!("{}:{}", name, passwd);
        let msg = format!("{}\x01\x00", cred);

        let mut konn = Konnection { stream: BufStream::new(sock),
                                    hp: hp,
                                    cred: cred,
                                    ver: 0,
        };

        konn.stream.write(msg.as_bytes()).unwrap();
        konn.stream.flush().unwrap();

        let mut resp = [0u8];
        let rd = konn.stream.read(&mut resp).unwrap();
        assert!(rd == 1);

        konn.ver = resp[0];

        Ok(konn)
    }

    pub fn query(&mut self, q: &str) -> i32 {
        let size = (mem::size_of::<KMessageHeader>()
            + mem::size_of::<KVectorHeader>()
            + q.len()) as i32;

        let mhdr = KMessageHeader { endian:1, msg_type:1, compress:0, unused:0, size:size };
        let vhdr = KVectorHeader { val_type:10, attrib:0, len:q.len() as i32 };

        self.stream.write_all(struct_to_bytes(&mhdr)).unwrap();
        self.stream.write_all(struct_to_bytes(&vhdr)).unwrap();
        self.stream.write_all(q.as_bytes()).unwrap();
        self.stream.flush().unwrap();

        size
    }

    pub fn response(&mut self) -> KObject {
        let mut mhdr: KMessageHeader = unsafe { mem::uninitialized() };
        read_all(&mut self.stream, struct_to_bytes_mut(&mut mhdr));
        println!("mhdr = {:?}", mhdr);
        self.read_kobj(&mhdr)
    }
    
    fn read_kobj(&mut self, mhdr: &KMessageHeader) -> KObject {
        let val_type = self.stream.fill_buf().unwrap()[0] as i8;
        let payload = mhdr.size - mem::size_of::<KMessageHeader>() as i32;
        println!("payload_size = {} val_type = {:?}", payload, val_type);
        match val_type {
            0 => self.read_klist(payload),
            1...19 => self.read_kvector(payload),
            -19...-1 => self.read_katom(payload),
            _ => self.read_kunknown(payload),
        }
    }

    fn read_atom<T>(&mut self) -> T {
        let mut a : T = unsafe { mem::uninitialized() };
        let ptr = (&mut a) as *mut _ as *mut u8;
        let siz = mem::size_of::<T>();
        unsafe {
            let slc = slice::from_raw_parts_mut(ptr, siz);
            read_all(&mut self.stream, slc);
        }
        a
    }

    fn read_vec<T>(&mut self, len: usize) -> Vec<T> {
        let mut v = Vec::<T>::with_capacity(len);
        let ptr = v.as_mut_ptr() as *mut u8;
        let size = len * mem::size_of::<T>();
        unsafe {
            let slc = slice::from_raw_parts_mut(ptr, size);
            read_all(&mut self.stream, slc);
            v.set_len(len);
        }
        v
    }

    fn read_klist(&mut self, size: i32) -> KObject {
        #![allow(unused_variables)]
        unimplemented!()
    }

    fn read_symatom(&mut self, size: usize) -> KObject {
        let mut v = Vec::<u8>::with_capacity(size);
        let ptr = v.as_mut_ptr() as *mut u8;
        let slc = unsafe { slice::from_raw_parts_mut(ptr, size) };
        read_all(&mut self.stream, slc);
        unsafe { v.set_len(size - 1); }
        print!("sym = {:?}", v);
        SymbolAtom(String::from_utf8(v).unwrap())
    }

    fn read_symvec(&mut self, size: usize) -> KObject {
        let mut v = Vec::<String>::new();
        {
            let stream = Read::take(&mut self.stream, size as u64 - 1u64);
            for s in stream.split(b'\0') {
                v.push(String::from_utf8(s.unwrap()).unwrap());
            }
        }
        // talchas is awesome. this bug along with demo code went up within half
        // an hour after I asked on #rust along with demo code.
        self.stream.consume(1);  // and the -1 seem to be a bug/poor interaction
                                 // between bufread, take, and split. Change Vec::split
                                 // https://github.com/rust-lang/rust/issues/27463
        SymbolVector(v)
    }

    fn read_kvector(&mut self, size: i32) -> KObject {
        let mut vhdr: KVectorHeader = unsafe { mem::uninitialized() };
        read_all(&mut self.stream, struct_to_bytes_mut(&mut vhdr));

        println!("vhdr = {:?}", vhdr);
        match vhdr.val_type {
            1 => BooleanVector(self.read_vec::<u8>(vhdr.len as usize)),
            2 => GuidVector(self.read_vec::<(u64, u64)>(vhdr.len as usize)),
            3 => unimplemented!(),
            4 => ByteVector(self.read_vec::<i8>(vhdr.len as usize)),
            5 => ShortVector(self.read_vec::<i16>(vhdr.len as usize)),
            6 => IntVector(self.read_vec::<i32>(vhdr.len as usize)),
            7 => LongVector(self.read_vec::<i64>(vhdr.len as usize)),
            8 => RealVector(self.read_vec::<f32>(vhdr.len as usize)),
            9 => FloatVector(self.read_vec::<f64>(vhdr.len as usize)),
            10 => CharVector(self.read_vec::<u8>(vhdr.len as usize)),
            11 => self.read_symvec(size as usize - mem::size_of::<KVectorHeader>()),
            12 => TimestampVector(self.read_vec::<i64>(vhdr.len as usize)),
            13 => MonthVector(self.read_vec::<i32>(vhdr.len as usize)),
            14 => DateVector(self.read_vec::<i32>(vhdr.len as usize)),
            15 => DateTimeVector(self.read_vec::<f64>(vhdr.len as usize)),
            16 => TimespanVector(self.read_vec::<i64>(vhdr.len as usize)),
            17 => MinuteVector(self.read_vec::<i32>(vhdr.len as usize)),
            18 => SecondVector(self.read_vec::<i32>(vhdr.len as usize)),
            19 => TimeVector(self.read_vec::<i32>(vhdr.len as usize)),
            _ => unreachable!(),
        }
    }

    fn read_katom(&mut self, size: i32) -> KObject {
        let mut buf = [0u8];
        read_all(&mut self.stream, buf.as_mut());

        let val_type = buf[0] as i8;
        println!("atom val_type = {}", val_type);
        match -val_type {
            1 => BooleanAtom(self.read_atom::<u8>()),
            2 => GuidAtom(self.read_atom::<(u64, u64)>()),
            3 => unimplemented!(),
            4 => ByteAtom(self.read_atom::<i8>()),
            5 => ShortAtom(self.read_atom::<i16>()),
            6 => IntAtom(self.read_atom::<i32>()),
            7 => LongAtom(self.read_atom::<i64>()),
            8 => RealAtom(self.read_atom::<f32>()),
            9 => FloatAtom(self.read_atom::<f64>()),
            10 => CharAtom(self.read_atom::<u8>()),
            11 => self.read_symatom(size as usize - 1usize),
            12 => TimestampAtom(self.read_atom::<i64>()),
            13 => MonthAtom(self.read_atom::<i32>()),
            14 => DateAtom(self.read_atom::<i32>()),
            15 => DateTimeAtom(self.read_atom::<f64>()),
            16 => TimespanAtom(self.read_atom::<i64>()),
            17 => MinuteAtom(self.read_atom::<i32>()),
            18 => SecondAtom(self.read_atom::<i32>()),
            19 => TimeAtom(self.read_atom::<i32>()),
            _ => unreachable!(),
        }
    }

    fn read_kunknown(&mut self, size: i32) -> KObject {
        UnknownObj(self.read_vec::<u8>(size as usize))
    }
}

