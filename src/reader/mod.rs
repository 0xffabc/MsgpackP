use std::io::{Cursor, Read};

use anyhow::Result;

use crate::{
    constants::Families,
    msgpack::{ReadFrom, array::Array, ext::Extension},
    value::Value,
};

enum Marker {
    FixStr,
    Nil,
    Boolean,
    Binary,
    Extension,
    Float32,
    Float64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    Array,
    FixArray,
    PosFixInt,
    NegFixInt,
    FixMap,
    Map,
    Unknown,
}

#[inline(always)]
fn classify_marker(packet_type: u8) -> Marker {
    match packet_type {
        _ if (Families::FIXSTR..=Families::FIXSTR + 0x1f).contains(&packet_type) => Marker::FixStr,
        Families::NIL | Families::RESERVED => Marker::Nil,
        Families::FALSE | Families::TRUE => Marker::Boolean,
        Families::BIN8 | Families::BIN16 | Families::BIN32 => Marker::Binary,
        Families::EXT8 | Families::EXT16 | Families::EXT32 => Marker::Extension,
        Families::FLOAT32 => Marker::Float32,
        Families::FLOAT64 => Marker::Float64,
        Families::UINT8 => Marker::UInt8,
        Families::UINT16 => Marker::UInt16,
        Families::UINT32 => Marker::UInt32,
        Families::UINT64 => Marker::UInt64,
        Families::INT8 => Marker::Int8,
        Families::INT16 => Marker::Int16,
        Families::INT32 => Marker::Int32,
        Families::INT64 => Marker::Int64,
        Families::STR8 | Families::STR16 | Families::STR32 => Marker::String,
        Array::ARRAY_16_TYPE | Array::ARRAY_32_TYPE => Marker::Array,
        _ if (Array::FIXARRAY_TYPE..=(Array::FIXARRAY_TYPE + 0x1f)).contains(&packet_type) => {
            Marker::FixArray
        }
        0x00..0x7f => Marker::PosFixInt,
        0xe0..=0xff => Marker::NegFixInt,
        Families::MAP16 | Families::MAP32 => Marker::Map,
        _ if (Families::FIXMAP_TYPE..=(Families::FIXMAP_TYPE + 0x0f)).contains(&packet_type) => {
            Marker::FixMap
        }
        _ => Marker::Unknown,
    }
}

#[inline(always)]
fn read_packet_type(reader: &mut Cursor<Box<[u8]>>) -> Result<u8> {
    let mut header = [0; 1];

    reader.read_exact(&mut header)?;

    Ok(header[0])
}

#[inline(always)]
pub fn read_value_from_cursor(reader: &mut Cursor<Box<[u8]>>) -> Result<Value> {
    let packet_type = read_packet_type(reader)?;
    let marker = classify_marker(packet_type);

    Ok(match marker {
        Marker::FixStr => Value::from(String::read_from(packet_type, reader)?),
        Marker::Nil => Value::Nil,
        Marker::Boolean => Value::from(bool::read_from(packet_type, reader)?),
        Marker::Binary => Value::from(
            Vec::<u8>::read_from(packet_type, reader)?
                .iter()
                .map(|&byte| Value::U8(byte))
                .collect::<Vec<_>>(),
        ),
        Marker::Extension => Value::from(Extension::read_from(packet_type, reader)?),
        Marker::Float32 => Value::from(f32::read_from(packet_type, reader)?),
        Marker::Float64 => Value::from(f64::read_from(packet_type, reader)?),
        Marker::UInt8 => Value::from(u8::read_from(packet_type, reader)?),
        Marker::UInt16 => Value::from(u16::read_from(packet_type, reader)?),
        Marker::UInt32 => Value::from(u32::read_from(packet_type, reader)?),
        Marker::UInt64 => Value::from(u64::read_from(packet_type, reader)?),
        Marker::Int8 => Value::from(i8::read_from(packet_type, reader)?),
        Marker::Int16 => Value::from(i16::read_from(packet_type, reader)?),
        Marker::Int32 => Value::from(i32::read_from(packet_type, reader)?),
        Marker::Int64 => Value::from(i64::read_from(packet_type, reader)?),
        Marker::String => Value::from(String::read_from(packet_type, reader)?),
        Marker::Array => Value::from(Vec::<Value>::read_from(packet_type, reader)?),
        Marker::FixArray => Value::from(Vec::<Value>::read_from(packet_type, reader)?),
        Marker::PosFixInt => Value::from(u8::read_from(packet_type, reader)?),
        Marker::NegFixInt => Value::from(i8::read_from(packet_type, reader)?),
        Marker::FixMap => Value::from(Vec::<(Value, Value)>::read_from(packet_type, reader)?),
        Marker::Map => Value::from(Vec::<(Value, Value)>::read_from(packet_type, reader)?),
        Marker::Unknown => Value::Nil,
    })
}
