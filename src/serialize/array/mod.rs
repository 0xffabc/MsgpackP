use std::io::Write;

use anyhow::Result;

use crate::serialize::{WriteTo, value::Value};

struct Array();

impl Array {
    pub const ARRAY_16_TYPE: u8 = 0xdc;
    pub const ARRAY_32_TYPE: u8 = 0xdd;
}

impl WriteTo for Vec<Value> {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let array_length = self.len();

        match array_length {
            0..=15 => buffer.write_all(&[0x90 + array_length as u8])?,
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
