use std::{fmt, io::Write};

use crate::{
    constants::Families,
    msgpack::{WriteTo, ext::Extension},
};
use anyhow::Result;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Nil,
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Str(String),
    Array(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Extension(Extension),
}

impl Value {
    #[inline(always)]
    pub fn nil() -> Self {
        Value::Nil
    }

    #[inline(always)]
    pub fn bool(value: bool) -> Self {
        Value::Bool(value)
    }

    #[inline(always)]
    pub fn u8(value: u8) -> Self {
        Value::U8(value)
    }

    #[inline(always)]
    pub fn u16(value: u16) -> Self {
        Value::U16(value)
    }

    #[inline(always)]
    pub fn u32(value: u32) -> Self {
        Value::U32(value)
    }

    #[inline(always)]
    pub fn u64(value: u64) -> Self {
        Value::U64(value)
    }

    #[inline(always)]
    pub fn f32(value: OrderedFloat<f32>) -> Self {
        Value::F32(value)
    }

    #[inline(always)]
    pub fn f64(value: OrderedFloat<f64>) -> Self {
        Value::F64(value)
    }

    #[inline(always)]
    pub fn i8(value: i8) -> Self {
        Value::I8(value)
    }

    #[inline(always)]
    pub fn i16(value: i16) -> Self {
        Value::I16(value)
    }

    #[inline(always)]
    pub fn i32(value: i32) -> Self {
        Value::I32(value)
    }

    #[inline(always)]
    pub fn i64(value: i64) -> Self {
        Value::I64(value)
    }

    #[inline(always)]
    pub fn str<T: Into<String>>(value: T) -> Self {
        Value::Str(value.into())
    }

    #[inline(always)]
    pub fn array<T: Into<Vec<Value>>>(value: T) -> Self {
        Value::Array(value.into())
    }

    #[inline(always)]
    pub fn map<T: Into<Vec<(Value, Value)>>>(value: T) -> Self {
        Value::Map(value.into())
    }

    #[inline(always)]
    pub fn extension(value: Extension) -> Self {
        Value::Extension(value)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "null"),
            Value::Bool(value) => write!(f, "{}", value),
            Value::U8(value) => write!(f, "{}", value),
            Value::U16(value) => write!(f, "{}", value),
            Value::U32(value) => write!(f, "{}", value),
            Value::U64(value) => write!(f, "{}", value),
            Value::I8(value) => write!(f, "{}", value),
            Value::I16(value) => write!(f, "{}", value),
            Value::I32(value) => write!(f, "{}", value),
            Value::I64(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Array(value) => write!(f, "{:?}", value),
            Value::Map(value) => write!(f, "{:?}", value),
            Value::Extension(value) => write!(f, "{:?}", value),
            Value::F32(value) => write!(f, "{}", value),
            Value::F64(value) => write!(f, "{}", value),
        }
    }
}

macro_rules! typed_to_value {
    ($($type:ty, $name:ident)?) => {
        $(
            impl From<$type> for Value {
                #[inline(always)]
                fn from(value: $type) -> Self {
                    Value::$name(value)
                }
            }
        )*
    };
}

impl From<f32> for Value {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Value::F32(OrderedFloat(value))
    }
}

impl From<f64> for Value {
    #[inline(always)]
    fn from(value: f64) -> Self {
        Value::F64(OrderedFloat(value))
    }
}

impl From<Extension> for Value {
    #[inline(always)]
    fn from(value: Extension) -> Self {
        Value::Extension(value)
    }
}

typed_to_value!(u8, U8);
typed_to_value!(u16, U16);
typed_to_value!(u32, U32);
typed_to_value!(u64, U64);
typed_to_value!(i8, I8);
typed_to_value!(i16, I16);
typed_to_value!(i32, I32);
typed_to_value!(i64, I64);
typed_to_value!(String, Str);
typed_to_value!(bool, Bool);
typed_to_value!(Vec<(Value, Value)>, Map);

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
            Value::Extension(value) => value.write_to(buffer)?,
        }
        Ok(())
    }
}
