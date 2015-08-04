// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

// TODO: Still not sure of the type mapping

use std::ptr::copy_nonoverlapping;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KReal(pub f32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KFloat(pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KChar(pub u8);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSymbol(pub String);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTimestamp(pub i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KMonth(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KDate(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KDateTime(pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTimespan(pub i64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KMinute(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSecond(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTime(pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KList(pub Vec<KObject>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KDictionary(pub KVector, pub KVector);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KTable(pub KVector, pub KList);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KKeyedTable(pub KTable, pub KTable);


unsafe fn make_vec<T>(data: *const T, len: usize) -> Vec<T> {
    let mut v = Vec::<T>::with_capacity(len);
    copy_nonoverlapping(data, v.as_mut_ptr(), len);
    v.set_len(len);
    v
}

