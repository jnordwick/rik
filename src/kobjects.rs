// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

// TODO: this needs to be reworked somehow. Enums are not the way
// to model this. Maybe use structs and phantom types to ensure
// type checking

use std::ptr::copy_nonoverlapping;

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

#[derive(Debug)]
pub struct KBoolean(pub u8);

#[derive(Debug)]
pub struct KGuid(pub u64, pub u64);

#[derive(Debug)]
pub struct KByte(pub i8);

#[derive(Debug)]
pub struct KShort(pub i16);

#[derive(Debug)]
pub struct KInt(pub i32);

#[derive(Debug)]
pub struct KLong(pub i64);

#[derive(Debug)]
pub struct KReal(pub f32);

#[derive(Debug)]
pub struct KFloat(pub f64);

#[derive(Debug)]
pub struct KChar(pub u8);

#[derive(Debug)]
pub struct KSymbol(pub String);

#[derive(Debug)]
pub struct KTimestamp(pub i64);

#[derive(Debug)]
pub struct KMonth(pub i32);

#[derive(Debug)]
pub struct KDate(pub i32);

#[derive(Debug)]
pub struct KDateTime(pub f64);

#[derive(Debug)]
pub struct KTimespan(pub i64);

#[derive(Debug)]
pub struct KMinute(pub i32);

#[derive(Debug)]
pub struct KSecond(pub i32);

#[derive(Debug)]
pub struct KTime(pub i32);

#[derive(Debug)]
pub struct KList(pub Vec<KObject>);

#[derive(Debug)]
pub struct KDictionary(pub KVector, pub KVector);

#[derive(Debug)]
pub struct KTable(pub KVector, pub KList);

#[derive(Debug)]
pub struct KKeyedTable(pub KTable, pub KTable);

#[derive(Debug)]
pub enum KObject {
    Atom        (KAtom       ),
    Vector      (KVector     ),

    Dictionary  (KDictionary),
    Table       (KTable),
    KeyedTable  (KKeyedTable),

    Function   (Vec<u8>),
    UnknownObj (Vec<u8>),
}

unsafe fn make_vec<T>(data: *const T, len: usize) -> Vec<T> {
    let mut v = Vec::<T>::with_capacity(len);
    copy_nonoverlapping(data, v.as_mut_ptr(), len);
    v.set_len(len);
    v
}

