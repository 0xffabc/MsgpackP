use std::io::Write;

use anyhow::Result;

use crate::{
    msgpack::{ReadFrom, WriteTo},
    reader::Reader,
    value::Value,
};

pub struct Array();

impl Array {
    pub const ARRAY_16_TYPE: u8 = 0xdc;
    pub const ARRAY_32_TYPE: u8 = 0xdd;
    pub const FIXARRAY_TYPE: u8 = 0x90;
}

impl WriteTo for Vec<Value<'_>> {
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

impl<'a> ReadFrom<'a> for Vec<Value<'a>> {
    #[inline(always)]
    fn read_from<U: AsRef<[u8]> + 'a>(array_type: u8, reader: &'a mut Reader<U>) -> Result<Self> {
        let array_length = match array_type {
            _ if ((0x90..=0x9f).contains(&array_type)) => (array_type - 0x90) as u32,
            Array::ARRAY_16_TYPE => {
                let bytes = reader.pull(2)?;
                u16::from_be_bytes([bytes[0], bytes[1]]) as u32
            }
            Array::ARRAY_32_TYPE => {
                let bytes = reader.pull(4)?;
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            }
            _ => return Ok(Vec::new()),
        };

        let mut values = Vec::with_capacity(array_length as usize);

        for _ in 0..array_length {
            unsafe {
                let reader_ptr = reader as *mut Reader<U> as *mut Reader<U>;
                let reader_ptr = &mut *reader_ptr;

                values.push(reader_ptr.pull_value().unwrap_or(Value::Nil));
            }
        }

        Ok(values)
    }
}
