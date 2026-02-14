use std::io::{Cursor, Read, Write};

use anyhow::Result;

use crate::{
    msgpack::{ReadFrom, WriteTo},
    reader::read_value_from_cursor,
    value::Value,
};

pub struct Array();

impl Array {
    pub const ARRAY_16_TYPE: u8 = 0xdc;
    pub const ARRAY_32_TYPE: u8 = 0xdd;
    pub const FIXARRAY_TYPE: u8 = 0x90;
}

impl WriteTo for Vec<Value> {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let array_length = self.len();

        match array_length {
            0..=15 => buffer.write_all(&[Array::FIXARRAY_TYPE + array_length as u8])?,
            16..=65535 => {
                buffer.write_all(&[Array::ARRAY_16_TYPE])?;
                buffer.write_all(&array_length.to_be_bytes())?
            }
            _ => {
                buffer.write_all(&[Array::ARRAY_32_TYPE])?;
                buffer.write_all(&array_length.to_be_bytes())?
            }
        }

        for value in self {
            value.write_to(buffer)?;
        }

        Ok(())
    }
}

impl ReadFrom for Vec<Value> {
    #[inline(always)]
    fn read_from(array_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let array_length = match array_type {
            _ if ((0x90..=0x9f).contains(&array_type)) => (array_type - 0x90) as u32,
            Array::ARRAY_16_TYPE => {
                let mut array_length_bytes = [0; 2];
                reader.read_exact(&mut array_length_bytes)?;
                u16::from_be_bytes(array_length_bytes) as u32
            }
            Array::ARRAY_32_TYPE => {
                let mut array_length_bytes = [0; 4];
                reader.read_exact(&mut array_length_bytes)?;
                u32::from_be_bytes(array_length_bytes)
            }
            _ => return Ok(Vec::new()),
        };

        let mut values = Vec::with_capacity(array_length as usize);

        for _ in 0..array_length {
            let value = read_value_from_cursor(reader)?;

            values.push(value);
        }

        Ok(values)
    }
}
