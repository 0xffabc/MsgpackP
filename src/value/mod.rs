use std::io::{Cursor, Read, Write};

use anyhow::Result;

use crate::{
    constants::Families,
    serialize::{ReadFrom, WriteTo, array::Array, ext::Extension},
};

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
    Extension(Extension),
}

pub fn read_value_from_cursor(reader: &mut Cursor<Vec<u8>>) -> Value {
    let mut header = [0; 1];

    reader.read_exact(&mut header).unwrap_or(());

    let packet_type = header[0];

    match packet_type {
        _ if (Families::FIXSTR..=Families::FIXSTR + 0x1f).contains(&packet_type) => {
            Value::Str(String::read_from(packet_type, reader))
        }
        Families::NIL | Families::RESERVED => Value::Nil,
        Families::FALSE | Families::TRUE => Value::Bool(bool::read_from(packet_type, reader)),
        Families::BIN8 | Families::BIN16 | Families::BIN32 => Value::Array(
            Vec::<u8>::read_from(packet_type, reader)
                .iter()
                .map(|&byte| Value::U8(byte))
                .collect::<Vec<_>>(),
        ),
        Families::FIXEXT1
        | Families::FIXEXT2
        | Families::FIXEXT4
        | Families::FIXEXT8
        | Families::FIXEXT16
        | Families::EXT8
        | Families::EXT16
        | Families::EXT32 => Value::Extension(Extension::read_from(packet_type, reader)),
        Families::FLOAT32 => Value::F32(f32::read_from(packet_type, reader)),
        Families::FLOAT64 => Value::F64(f64::read_from(packet_type, reader)),
        Families::UINT8 => Value::U8(u8::read_from(packet_type, reader)),
        Families::UINT16 => Value::U16(u16::read_from(packet_type, reader)),
        Families::UINT32 => Value::U32(u32::read_from(packet_type, reader)),
        Families::UINT64 => Value::U64(u64::read_from(packet_type, reader)),
        Families::INT8 => Value::I8(i8::read_from(packet_type, reader)),
        Families::INT16 => Value::I16(i16::read_from(packet_type, reader)),
        Families::INT32 => Value::I32(i32::read_from(packet_type, reader)),
        Families::INT64 => Value::I64(i64::read_from(packet_type, reader)),
        Families::STR8 | Families::STR16 | Families::STR32 => {
            Value::Str(String::read_from(packet_type, reader))
        }
        Array::ARRAY_16_TYPE | Array::ARRAY_32_TYPE => {
            Value::Array(Vec::<Value>::read_from(packet_type, reader))
        }
        _ if (Array::FIXARRAY_TYPE..=(Array::FIXARRAY_TYPE + 0x1f)).contains(&packet_type) => {
            Value::Array(Vec::<Value>::read_from(packet_type, reader))
        }
        0x00..0x7f => Value::U8(u8::read_from(packet_type, reader)),
        0xe0..=0xff => Value::I8(i8::read_from(packet_type, reader)),
        _ => Value::Nil,
    }
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
            Value::Extension(value) => value.write_to(buffer)?,
        }
        Ok(())
    }
}
