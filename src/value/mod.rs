use std::{fmt, io::Write};

use crate::{
    constants::Families,
    msgpack::{WriteTo, ext::Extension},
};

use anyhow::Result;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value<'a> {
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
    Str(&'a str),
    Array(Box<[Value<'a>]>),
    Map(Box<[(Value<'a>, Value<'a>)]>),
    Extension(Extension),
}

impl<'a> Value<'a> {
    #[inline(always)]
    pub fn nil() -> Self {
        Value::Nil
    }

    /**
     * @name bin
     * @description According to msgpack spec, Extension is built
     * from a Vec<u8>
     *
     * I assume that moomoo.io's server doesn't use any extensions whatsoever.
     */
    #[inline(always)]
    pub fn bin(value: Vec<u8>) -> Self {
        Value::Extension(Extension::new(Families::BIN8, value))
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

    /**
     * @name str
     * @description
     *
     * Important note: You likely don't want to make a `str` reference that points
     * to local variable, since it would trigger the borrow checker,
     * so obviously pull the strings from the buffer you were given.
     */
    #[inline(always)]
    pub fn str(value: &'a str) -> Self {
        Value::Str(value)
    }

    #[inline(always)]
    pub fn array(value: Box<[Value<'a>]>) -> Self {
        Value::Array(value)
    }

    #[inline(always)]
    pub fn map(value: Box<[(Value<'a>, Value<'a>)]>) -> Self {
        Value::Map(value)
    }

    #[inline(always)]
    pub fn extension(value: Extension) -> Self {
        Value::Extension(value)
    }
}

impl fmt::Display for Value<'_> {
    /**
     * @name fmt
     * @description
     *
     * Debugging utilities
     */
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

impl WriteTo for Value<'_> {
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
            Value::Str(value) => value.to_owned().to_owned().write_to(buffer)?,
            Value::Array(value) => value.write_to(buffer)?,
            Value::Map(value) => value.write_to(buffer)?,
            Value::Nil => buffer.write_all(&Families::NIL.to_be_bytes())?,
            Value::Bool(value) => value.write_to(buffer)?,
            Value::Extension(value) => value.write_to(buffer)?,
        }

        Ok(())
    }
}
