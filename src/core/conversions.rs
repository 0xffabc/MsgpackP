pub struct Serialized(Vec<u8>);

pub enum Value {
    Nil,
    Reserved,
    Bool(bool),
    IntegerU8(u8),
    IntegerU16(u16),
    IntegerU32(u32),
    IntegerU64(u64),
    Float32(f32),
    Float64(f64),
    IntegerI8(i8),
    IntegerI16(i16),
    IntegerI32(i32),
    IntegerI64(i64),
    String(String),
}

pub mod float;
pub mod signed;
pub mod string;
pub mod unsigned;
