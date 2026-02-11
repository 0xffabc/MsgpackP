use std::io::Write;

use anyhow::Result;

use crate::{serialize::WriteTo, value::Value};

struct Map();

impl Map {
    pub const MAP_16_TYPE: u8 = 0xde;
    pub const MAP_32_TYPE: u8 = 0xdf;
}

impl WriteTo for Vec<(Value, Value)> {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let map_length = self.len();

        match map_length {
            0..=15 => buffer.write_all(&[0x80 + map_length as u8])?,
            16..=65535 => {
                buffer.write_all(&[Map::MAP_16_TYPE])?;
                buffer.write_all(&map_length.to_be_bytes())?
            }
            _ => {
                buffer.write_all(&[Map::MAP_32_TYPE])?;
                buffer.write_all(&map_length.to_be_bytes())?
            }
        }

        for (key, value) in self {
            key.write_to(buffer)?;
            value.write_to(buffer)?;
        }

        Ok(())
    }
}
