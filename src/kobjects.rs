// For now just the KObject enum. will including parsing code
// as it gets extracted from the protocol code

#[derive(Debug)]
pub enum KObject {
    // Atoms
    BooleanAtom   ( u8       ),
    GuidAtom      ( (u64, u64) ),
    ByteAtom      ( i8       ),
    ShortAtom     ( i16      ),
    IntAtom       ( i32      ),
    LongAtom      ( i64      ),
    RealAtom      ( f32      ),
    FloatAtom     ( f64      ),
    CharAtom      ( u8       ),
    SymbolAtom    ( String   ),
    TimestampAtom ( i64      ),
    MonthAtom     ( i32      ),
    DateAtom      ( i32      ),
    DateTimeAtom  ( f64      ),
    TimespanAtom  ( i64      ),
    MinuteAtom    ( i32      ),
    SecondAtom    ( i32      ),
    TimeAtom      ( i32      ),

    // Vectors
    GeneralList     ( Vec<KObject>    ),
    BooleanVector   ( Vec<u8>         ),
    GuidVector      ( Vec<(u64, u64)> ),
    ByteVector      ( Vec<i8>         ),
    ShortVector     ( Vec<i16>        ),
    IntVector       ( Vec<i32>        ),
    LongVector      ( Vec<i64>        ),
    RealVector      ( Vec<f32>        ),
    FloatVector     ( Vec<f64>        ),
    CharVector      ( Vec<u8>         ),
    SymbolVector    ( Vec<String>     ),
    TimestampVector ( Vec<i64>        ),
    MonthVector     ( Vec<i32>        ),
    DateVector      ( Vec<i32>        ),
    DateTimeVector  ( Vec<f64>        ),
    TimespanVector  ( Vec<i64>        ),
    MinuteVector    ( Vec<i32>        ),
    SecondVector    ( Vec<i32>        ),
    TimeVector      ( Vec<i32>        ),

    // Other
    UnknownObj ( Vec<u8> ),
}


// enum KDict {
//     Dict       ( KVector, KVector ),
//     SortedDict ( KVector, KVector ),
//     Table      ( KVector, KVector ),
// }

// enum KKEyed {
//     KeyedTable       ( KDict, KDict ),
//     SortedKeyedTable ( KDict, KDict ),
// }

// struct DictRepr<K,V> {
//     keys: Vec<K>,
//     vals: Vec<V>,

// }
// enum KVerbPhrase {
//     KVerb,
//     KAdverb,
// }

// enum KVerb {
//     Lambda              ( String ),
//     UnaryPrimitive      ( u8     ),
//     BinaryPrimitive     ( u8     ),
//     TernaryPrimiive     ( u8     ),
//     FunctionProjection  ( Vec<KVerbPhrase> ),
//     FunctionComposition ( Vec<KVerbPhrase> ),
// }

// enum KAdverb {
//     EachAdverb      ( KVerbPhrase ),
//     OverAdverb      ( KVerbPhrase ),
//     ScanAdverb      ( KVerbPhrase ),
//     EachPairAdverb  ( KVerbPhrase ),
//     RightEachAdverb ( KVerbPhrase ),
//     LeftEachAdverb  ( KVerbPhrase ),
// }
