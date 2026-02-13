use std::io::{Cursor, Read};

use anyhow::Result;

use crate::{
    constants::Families,
    msgpack::{ReadFrom, array::Array, ext::Extension},
    value::Value,
};

#[inline(always)]
pub fn read_value_from_cursor(reader: &mut Cursor<Vec<u8>>) -> Result<Value> {
    let mut header = [0; 1];

    reader.read_exact(&mut header)?;

    let packet_type = header[0];

    Ok(match packet_type {
        _ if (Families::FIXSTR..=Families::FIXSTR + 0x1f).contains(&packet_type) => {
            Value::from(String::read_from(packet_type, reader)?)
        }
        Families::NIL | Families::RESERVED => Value::Nil,
        Families::FALSE | Families::TRUE => Value::from(bool::read_from(packet_type, reader)?),
        Families::BIN8 | Families::BIN16 | Families::BIN32 => Value::from(
            Vec::<u8>::read_from(packet_type, reader)?
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
        | Families::EXT32 => Value::from(Extension::read_from(packet_type, reader)?),
        Families::FLOAT32 => Value::from(f32::read_from(packet_type, reader)?),
        Families::FLOAT64 => Value::from(f64::read_from(packet_type, reader)?),
        Families::UINT8 => Value::from(u8::read_from(packet_type, reader)?),
        Families::UINT16 => Value::from(u16::read_from(packet_type, reader)?),
        Families::UINT32 => Value::from(u32::read_from(packet_type, reader)?),
        Families::UINT64 => Value::from(u64::read_from(packet_type, reader)?),
        Families::INT8 => Value::from(i8::read_from(packet_type, reader)?),
        Families::INT16 => Value::from(i16::read_from(packet_type, reader)?),
        Families::INT32 => Value::from(i32::read_from(packet_type, reader)?),
        Families::INT64 => Value::from(i64::read_from(packet_type, reader)?),
        Families::STR8 | Families::STR16 | Families::STR32 => {
            Value::from(String::read_from(packet_type, reader)?)
        }
        Array::ARRAY_16_TYPE | Array::ARRAY_32_TYPE => {
            Value::from(Vec::<Value>::read_from(packet_type, reader)?)
        }
        _ if (Array::FIXARRAY_TYPE..=(Array::FIXARRAY_TYPE + 0x1f)).contains(&packet_type) => {
            Value::from(Vec::<Value>::read_from(packet_type, reader)?)
        }
        0x00..0x7f => Value::from(u8::read_from(packet_type, reader)?),
        0xe0..=0xff => Value::from(i8::read_from(packet_type, reader)?),
        _ if (Families::FIXMAP_TYPE..=(Families::FIXMAP_TYPE + 0x0f)).contains(&packet_type) => {
            Value::from(Vec::<(Value, Value)>::read_from(packet_type, reader)?)
        }
        Families::MAP16 | Families::MAP32 => {
            Value::from(Vec::<(Value, Value)>::read_from(packet_type, reader)?)
        }
        127..=191 => Value::Nil,
    })
}
