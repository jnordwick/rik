// TODO: tables, dicts
// TODO: remove header structs?
// TODO: maps to standard rust collections
// TODO: serialize to kdb
// TODO: functions
// TODO: collapse all atom/vector out, use guards on type codes instead?

use std::mem::size_of;
use std::ptr::{read, copy_nonoverlapping};
use std::vec::Vec;

#[derive(Debug)]
pub enum KObject {
    Atom        (KAtom),
    Vector      (KVector),

    Dictionary  (KDictionary),
    Table       (KTable),
    KeyedTable  (KKeyedTable),

    Lambda   (KLambda),
    Unknown  (Vec<u8>),
}

#[derive(Debug)]
pub enum KAtom {
    Boolean   (KBoolean),
    Guid      (KGuid),
    Byte      (KByte),
    Short     (KShort),
    Int       (KInt),
    Long      (KLong),
    Real      (KReal),
    Float     (KFloat),
    Char      (KChar),
    Symbol    (KSymbol),
    Timestamp (KTimestamp),
    Month     (KMonth),
    Date      (KDate),
    DateTime  (KDateTime),
    Timespan  (KTimespan),
    Minute    (KMinute),
    Second    (KSecond),
    Time      (KTime),
}

#[derive(Debug)]
pub enum KVector {
    List(KList),

    Boolean   (Vec<KBoolean>),
    Guid      (Vec<KGuid>),
    Byte      (Vec<KByte>),
    Short     (Vec<KShort>),
    Int       (Vec<KInt>),
    Long      (Vec<KLong>),
    Real      (Vec<KReal>),
    Float     (Vec<KFloat>),
    Char      (Vec<KChar>),
    Symbol    (Vec<KSymbol>),
    Timestamp (Vec<KTimestamp>),
    Month     (Vec<KMonth>),
    Date      (Vec<KDate>),
    DateTime  (Vec<KDateTime>),
    Timespan  (Vec<KTimespan>),
    Minute    (Vec<KMinute>),
    Second    (Vec<KSecond>),
    Time      (Vec<KTime>),
}

impl KVector {
    pub fn len(&self) -> usize {
        match *self {
            KVector::List(ref v) => v.len(),
            KVector::Boolean(ref v) => v.len(),
            KVector::Guid(ref v) => v.len(),
            KVector::Byte(ref v) => v.len(),
            KVector::Short(ref v) => v.len(),
            KVector::Int(ref v) => v.len(),
            KVector::Long(ref v) => v.len(),
            KVector::Real(ref v) => v.len(),
            KVector::Float(ref v) => v.len(),
            KVector::Char(ref v) => v.len(),
            KVector::Symbol(ref v) => v.len(),
            KVector::Timestamp(ref v) => v.len(),
            KVector::Month(ref v) => v.len(),
            KVector::Date(ref v) => v.len(),
            KVector::DateTime(ref v) => v.len(),
            KVector::Timespan(ref v) => v.len(),
            KVector::Minute(ref v) => v.len(),
            KVector::Second(ref v) => v.len(),
            KVector::Time(ref v) => v.len(),
        }
    }
}

pub type KBoolean = u8;
pub type KGuid = [u64;16];
pub type KByte = i8;
pub type KShort = i16;
pub type KInt = i32;
pub type KLong = i64;
pub type KReal = f32;
pub type KFloat = f64;
pub type KChar = u8;
pub type KSymbol = String;
pub type KTimestamp = i64;
pub type KMonth = i32;
pub type KDate = i32;
pub type KDateTime = f64;
pub type KTimespan = i64;
pub type KMinute = i32;
pub type KSecond = i32;
pub type KTime = i32;
pub type KList = Vec<KObject>;
pub type KLambda = (KSymbol, String);

#[derive(Debug)]
pub struct KDictionary(pub KVector, pub KVector);

#[derive(Debug)]
pub struct KTable(pub KVector, pub KList);

#[derive(Debug)]
pub struct KKeyedTable(pub KTable, pub KTable);


// #[derive(Debug)]
// #[repr(packed)]
// struct KAtomHeader {
//     val_type: i8,
// }

#[derive(Debug)]
#[repr(packed)]
struct KVectorHeader {
    val_type: i8,
    attrib: i8,
    len: i32,
}

// #[derive(Debug)]
// #[repr(packed)]
// struct KTableHeader {
//     val_type: i8, // = 98
//     attrib: i8,
// }

macro_rules! cast_add {
    ($c:expr, $p:expr, $i:expr) => ({
        let (val, len) = $p;
        ($c(val), len + $i)
    })
}

impl KObject {

    pub fn parse(msg: &[u8]) -> (KObject, usize) {
        let val_type = msg[0] as i8;
        match val_type {
            -19...-1 => cast_add!(KObject::Atom, Self::parse_atom(msg), 0),
            0...19 => cast_add!(KObject::Vector, Self::parse_vector(msg), 0),
            98 => cast_add!(KObject::Table, Self::parse_table(msg), 0),
            99 | 127 => Self::parse_dict(msg),
            100 => cast_add!(KObject::Lambda, Self::parse_lambda(msg), 0),
            _ => unimplemented!(),
        }
    }

    fn parse_dict(msg: &[u8]) -> (KObject, usize) {
        let (keys, klen) = Self::parse(&msg[1..]);
        let (vals, vlen) = Self::parse(&msg[1+klen..]);
        let kobj = match (keys, vals) {
            (KObject::Vector(kv), KObject::Vector(vv)) => {
                KObject::Dictionary(KDictionary(kv, vv))
            }
            (KObject::Table(kt), KObject::Table(vt)) => {
                KObject::KeyedTable(KKeyedTable(kt, vt))
            }
            _ => {
                unimplemented!();
            }
        };
        (kobj, 1+klen+vlen)
    }

    fn parse_table(msg: &[u8]) -> (KTable, usize) {
        let (dict, len) = Self::parse_dict(&msg[2..]);
        match dict {
            KObject::Dictionary(KDictionary(v, KVector::List(w))) => (KTable(v, w), 2+len),
            _ => unreachable!()
        }
    }

    fn parse_lambda(msg: &[u8]) -> (KLambda, usize) {
        let (sym, slen) = Self::read_sym_atom(&msg[1..]);
        let (text, tlen) = Self::parse_vector(&msg[1+slen..]);
        match text {
            KVector::Char(v) => ((sym, String::from_utf8(v).unwrap()), 1+slen+tlen),
            _ => unimplemented!(),
        }
    }

    fn parse_atom(msg: &[u8]) -> (KAtom, usize) {
        let val_type = msg[0]; // Could cast to KAtomHeader, but why bother?
        let val = &msg[1..];
        match val_type as i8 {
            -19 => cast_add!(KAtom::Time, Self::read_atom::<KTime>(val), 1),
            -18 => cast_add!(KAtom::Second, Self::read_atom::<KSecond>(val), 1),
            -17 => cast_add!(KAtom::Minute, Self::read_atom::<KMinute>(val), 1),
            -16 => cast_add!(KAtom::Timespan, Self::read_atom::<KTimespan>(val), 1),
            -15 => cast_add!(KAtom::DateTime, Self::read_atom::<KDateTime>(val), 1),
            -14 => cast_add!(KAtom::Date, Self::read_atom::<KDate>(val), 1),
            -13 => cast_add!(KAtom::Month, Self::read_atom::<KMonth>(val), 1),
            -12 => cast_add!(KAtom::Timestamp, Self::read_atom::<KTimestamp>(val), 1),
            -11 => cast_add!(KAtom::Symbol, Self::read_sym_atom(val), 1),
            -10 => cast_add!(KAtom::Char, Self::read_atom::<KChar>(val), 1),
            -9 => cast_add!(KAtom::Float, Self::read_atom::<KFloat>(val), 1),
            -8 => cast_add!(KAtom::Real, Self::read_atom::<KReal>(val), 1),
            -7 => cast_add!(KAtom::Long, Self::read_atom::<KLong>(val), 1),
            -6 => cast_add!(KAtom::Int, Self::read_atom::<KInt>(val), 1),
            -5 => cast_add!(KAtom::Short, Self::read_atom::<KShort>(val), 1),
            -4 => cast_add!(KAtom::Byte, Self::read_atom::<KByte>(val), 1),
            -2 => cast_add!(KAtom::Guid, Self::read_atom::<KGuid>(val), 1),
            -1 => cast_add!(KAtom::Boolean, Self::read_atom::<KBoolean>(val), 1),
            _ => unreachable!(),
        }
    }

    fn parse_vector(msg: &[u8]) -> (KVector, usize) {
        let size = size_of::<KVectorHeader>();
        let (vhdr, _) : (KVectorHeader, _) = Self::read_atom(&msg[..size]);
        let val = &msg[size..];
        match vhdr.val_type {
            19 => cast_add!(KVector::Time, Self::read_vector::<KTime>(vhdr.len, val), size),
            18 => cast_add!(KVector::Second, Self::read_vector::<KSecond>(vhdr.len, val), size),
            17 => cast_add!(KVector::Minute, Self::read_vector::<KMinute>(vhdr.len, val), size),
            16 => cast_add!(KVector::Timespan, Self::read_vector::<KTimespan>(vhdr.len, val), size),
            15 => cast_add!(KVector::DateTime, Self::read_vector::<KDateTime>(vhdr.len, val), size),
            14 => cast_add!(KVector::Date, Self::read_vector::<KDate>(vhdr.len, val), size),
            13 => cast_add!(KVector::Month, Self::read_vector::<KMonth>(vhdr.len, val), size),
            12 => cast_add!(KVector::Timestamp, Self::read_vector::<KTimestamp>(vhdr.len, val), size),
            11 => cast_add!(KVector::Symbol, Self::read_sym_vector(vhdr.len, val), size),
            10 => cast_add!(KVector::Char, Self::read_vector::<KChar>(vhdr.len, val), size),
            9 => cast_add!(KVector::Float, Self::read_vector::<KFloat>(vhdr.len, val), size),
            8 => cast_add!(KVector::Real, Self::read_vector::<KReal>(vhdr.len, val), size),
            7 => cast_add!(KVector::Long, Self::read_vector::<KLong>(vhdr.len, val), size),
            6 => cast_add!(KVector::Int, Self::read_vector::<KInt>(vhdr.len, val), size),
            5 => cast_add!(KVector::Short, Self::read_vector::<KShort>(vhdr.len, val), size),
            4 => cast_add!(KVector::Byte, Self::read_vector::<KByte>(vhdr.len, val), size),
            2 => cast_add!(KVector::Guid, Self::read_vector::<KGuid>(vhdr.len, val), size),
            1 => cast_add!(KVector::Boolean, Self::read_vector::<KBoolean>(vhdr.len, val), size),
            0 => cast_add!(KVector::List, Self::read_list(vhdr.len, val), size),
            _ => unreachable!(),
        }
    }

    fn read_list(len: i32, msg: &[u8]) -> (KList, usize) {
        let mut v = Vec::<KObject>::with_capacity(len as usize);
        let mut s = 0usize;
        for _ in 0..len {
            let (obj, len) = Self::parse(&msg[s..]);
            v.push(obj);
            s += len;
        }
        (v, s)
    }

    fn read_atom<T>(data: &[u8]) -> (T, usize) {
        unsafe { (read(data.as_ptr() as *const T), size_of::<T>()) }
    }

    // FIXME: hacky. If not 0-terminated, gives back from  length (one too long),
    //  but still works bc the ony caller doesn't add term byte either!
    fn read_sym_atom(data: &[u8]) -> (KSymbol, usize) {
        let p = data.iter().position(|x| *x == 0u8).unwrap();
        let s = String::from_utf8(data[..p].to_vec()).unwrap();
        (s, p + 1)
    }

    fn read_vector<T>(len: i32, data: &[u8]) -> (Vec<T>, usize) {
        let mut v = Vec::<T>::with_capacity(len as usize);
        let size = size_of::<T>() * len as usize;
        unsafe {
            copy_nonoverlapping(data.as_ptr(), v.as_mut_ptr() as *mut u8, size);
            v.set_len(len as usize);
        }
        (v, size)
    }

    fn read_sym_vector(len: i32, data: &[u8]) -> (Vec<KSymbol>, usize) {
        let mut v = Vec::<KSymbol>::with_capacity(len as usize);
        let mut s = 0usize;
        while v.len() < len as usize {
            let e = data[s..].iter().position(|x| *x == 0u8).unwrap();
            v.push(String::from_utf8(data[s..s+e].to_vec()).unwrap());
            s += e + 1;
        }
        (v, s)
    }
}

