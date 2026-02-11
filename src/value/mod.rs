use std::io::Write;

use anyhow::Result;

use crate::{constants::Families, serialize::WriteTo};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Str(String),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
}

macro_rules! typed_to_value {
    ($($type:ty, $name:ident)?) => {
        $(
            impl From<$type> for Value {
                fn from(value: $type) -> Self {
                    Value::$name(value)
                }
            }
        )*
    };
}

typed_to_value!(u8, U8);
typed_to_value!(u16, U16);
typed_to_value!(u32, U32);
typed_to_value!(u64, U64);
typed_to_value!(f32, F32);
typed_to_value!(f64, F64);
typed_to_value!(i8, I8);
typed_to_value!(i16, I16);
typed_to_value!(i32, I32);
typed_to_value!(i64, I64);
typed_to_value!(String, Str);

impl From<Vec<Value>> for Value {
    #[inline(always)]
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl WriteTo for Value {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        match self {
            Value::U8(value) => value.write_to(buffer)?,
            Value::U16(value) => value.write_to(buffer)?,
            Value::U32(value) => value.write_to(buffer)?,
            Value::U64(value) => value.write_to(buffer)?,
            Value::F32(value) => value.write_to(buffer)?,
            Value::F64(value) => value.write_to(buffer)?,
            Value::I8(value) => value.write_to(buffer)?,
            Value::I16(value) => value.write_to(buffer)?,
            Value::I32(value) => value.write_to(buffer)?,
            Value::I64(value) => value.write_to(buffer)?,
            Value::Str(value) => value.write_to(buffer)?,
            Value::Array(value) => value.write_to(buffer)?,
            Value::Map(value) => value.write_to(buffer)?,
            Value::Nil => buffer.write_all(&Families::NIL.to_be_bytes())?,
            Value::Bool(value) => value.write_to(buffer)?,
        }
        Ok(())
    }
}
