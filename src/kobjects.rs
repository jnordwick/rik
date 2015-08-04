// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

#[derive(Debug)]
pub enum KVector {
    Boolean   (Vec<u8>        ),
    Guid      (Vec<(u64, u64)>),
    Byte      (Vec<i8>        ),
    Short     (Vec<i16>       ),
    Int       (Vec<i32>       ),
    Long      (Vec<i64>       ),
    Real      (Vec<f32>       ),
    Float     (Vec<f64>       ),
    Char      (Vec<u8>        ),
    Symbol    (Vec<String>    ),
    Timestamp (Vec<i64>       ),
    Month     (Vec<i32>       ),
    Date      (Vec<i32>       ),
    DateTime  (Vec<f64>       ),
    Timespan  (Vec<i64>       ),
    Minute    (Vec<i32>       ),
    Second    (Vec<i32>       ),
    Time      (Vec<i32>       ),
}

#[derive(Debug)]
pub enum KAtom {
    Boolean   (u8        ),
    Guid      ((u64, u64)),
    Byte      (i8        ),
    Short     (i16       ),
    Int       (i32       ),
    Long      (i64       ),
    Real      (f32       ),
    Float     (f64       ),
    Char      (u8        ),
    Symbol    (String    ),
    Timestamp (i64       ),
    Month     (i32       ),
    Date      (i32       ),
    DateTime  (f64       ),
    Timespan  (i64       ),
    Minute    (i32       ),
    Second    (i32       ),
    Time      (i32       ),
}

#[derive(Debug)]
pub enum KObject {
    // Atoms
    GeneralList (Vec<KObject>),
    Atom        (KAtom       ),
    Vector      (KVector     ),

    // Composites
    // Dict              ((KObject, KObject)),
    // SortedDict        ((KObject, KObject)),
    // Table             ((KObject, KObject)),

    // // TODO: These are really (Table, Table)
    // KeyedTable        ((KObject, KObject)),
    // SortedeKeyedTable ((KObject, KObject)),

    // Other
    // TODO: maybe parse functions propertly?
    Function   (Vec<u8>),
    UnknownObj (Vec<u8>),
}

