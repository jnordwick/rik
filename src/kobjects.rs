// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

// TODO: Still not sure of the type mapping

use std::mem::size_of;
use std::ptr::{read, copy_nonoverlapping};

#[derive(Debug)]
pub enum KObject {
    Atom        (KAtom),
    Vector      (KVector),

    Dictionary  (KDictionary),
    Table       (KTable),
    KeyedTable  (KKeyedTable),

    Function   (Vec<u8>),
    UnknownObj (Vec<u8>),
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
    GeneralList(KList),

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KBoolean(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KGuid(pub [u64;16]);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KByte(pub i8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KShort(pub i16);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KInt(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KLong(pub i64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct KReal(pub f32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct KFloat(pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KChar(pub u8);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSymbol(pub String);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTimestamp(pub i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KMonth(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KDate(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct KDateTime(pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTimespan(pub i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KMinute(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSecond(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTime(pub i32);

#[derive(Debug)]
pub struct KList(pub Vec<KObject>);

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

impl KObject {

    pub fn parse(msg: &Vec<u8>) -> KObject {
        let val_type = msg[0] as i8;
        println!("msg size = {} val type = {}", msg.len(), val_type);
        match val_type {
            -19...-1 => KObject::Atom(Self::parse_atom(msg)),
            0 => unimplemented!(),
            1...19 => KObject::Vector(Self::parse_vector(msg)),
            _ => unimplemented!(),
        }
    }

    fn parse_atom(msg: &Vec<u8>) -> KAtom {
        let val_type = msg[0]; // Could cast to KAtomHeader, but why bother?
        let val = &msg[1..];
        match val_type as i8 {
            -19 => KAtom::Time(Self::read_atom::<KTime>(val)),
            -18 => KAtom::Second(Self::read_atom::<KSecond>(val)),
            -17 => KAtom::Minute(Self::read_atom::<KMinute>(val)),
            -16 => KAtom::Timespan(Self::read_atom::<KTimespan>(val)),
            -15 => KAtom::DateTime(Self::read_atom::<KDateTime>(val)),
            -14 => KAtom::Date(Self::read_atom::<KDate>(val)),
            -13 => KAtom::Month(Self::read_atom::<KMonth>(val)),
            -12 => KAtom::Timestamp(Self::read_atom::<KTimestamp>(val)),
            -11 => KAtom::Symbol(Self::read_sym_atom(val)),
            -10 => KAtom::Char(Self::read_atom::<KChar>(val)),
            -9 => KAtom::Float(Self::read_atom::<KFloat>(val)),
            -8 => KAtom::Real(Self::read_atom::<KReal>(val)),
            -7 => KAtom::Long(Self::read_atom::<KLong>(val)),
            -6 => KAtom::Int(Self::read_atom::<KInt>(val)),
            -5 => KAtom::Short(Self::read_atom::<KShort>(val)),
            -4 => KAtom::Byte(Self::read_atom::<KByte>(val)),
            -3 => unimplemented!(),
            -2 => KAtom::Guid(Self::read_atom::<KGuid>(val)),
            -1 => KAtom::Boolean(Self::read_atom::<KBoolean>(val)),
            _ => unreachable!(),
        }
    }

    fn parse_vector(msg: &Vec<u8>) -> KVector {
        let size = size_of::<KVectorHeader>();
        let vhdr : KVectorHeader = Self::read_atom(&msg[..size]);
        let val = &msg[size..];
        match vhdr.val_type {
            19 => KVector::Time(Self::read_vector::<KTime>(vhdr.len, val)),
            18 => KVector::Second(Self::read_vector::<KSecond>(vhdr.len, val)),
            17 => KVector::Minute(Self::read_vector::<KMinute>(vhdr.len, val)),
            16 => KVector::Timespan(Self::read_vector::<KTimespan>(vhdr.len, val)),
            15 => KVector::DateTime(Self::read_vector::<KDateTime>(vhdr.len, val)),
            14 => KVector::Date(Self::read_vector::<KDate>(vhdr.len, val)),
            13 => KVector::Month(Self::read_vector::<KMonth>(vhdr.len, val)),
            12 => KVector::Timestamp(Self::read_vector::<KTimestamp>(vhdr.len, val)),
            11 => KVector::Symbol(Self::read_sym_vector(vhdr.len, val)),
            10 => KVector::Char(Self::read_vector::<KChar>(vhdr.len, val)),
            9 => KVector::Float(Self::read_vector::<KFloat>(vhdr.len, val)),
            8 => KVector::Real(Self::read_vector::<KReal>(vhdr.len, val)),
            7 => KVector::Long(Self::read_vector::<KLong>(vhdr.len, val)),
            6 => KVector::Int(Self::read_vector::<KInt>(vhdr.len, val)),
            5 => KVector::Short(Self::read_vector::<KShort>(vhdr.len, val)),
            4 => KVector::Byte(Self::read_vector::<KByte>(vhdr.len, val)),
            3 => unimplemented!(),
            2 => KVector::Guid(Self::read_vector::<KGuid>(vhdr.len, val)),
            1 => KVector::Boolean(Self::read_vector::<KBoolean>(vhdr.len, val)),
            _ => unreachable!(),
        }
    }

    fn read_atom<T>(data: &[u8]) -> T {
        unsafe { read(data.as_ptr() as *const T) }
    }

    fn read_sym_atom(data: &[u8]) -> KSymbol {
        KSymbol(String::from_utf8(data[..data.len()-1].to_vec()).unwrap())
    }

    fn read_vector<T>(len: i32, data: &[u8]) -> Vec<T> {
        assert!(len as usize * size_of::<T>() == data.len());
        let mut v = Vec::<T>::with_capacity(len as usize);
        unsafe {
            copy_nonoverlapping(data.as_ptr(), v.as_mut_ptr() as *mut u8, data.len());
            v.set_len(len as usize);
        }
        v
    }

    fn read_sym_vector(len: i32, data: &[u8]) -> Vec<KSymbol> {
        let mut v = Vec::<KSymbol>::with_capacity(len as usize);
        for sub in data.split(|e| *e == 0) {
            v.push(Self::read_sym_atom(sub));
        }
        v
    }
}

