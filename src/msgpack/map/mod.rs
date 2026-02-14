use std::io::Write;

use anyhow::Result;

use crate::{
    msgpack::{ReadFrom, WriteTo},
    reader::Reader,
    value::Value,
};

struct Map();

impl Map {
    pub const FIXMAP: u8 = 0x80;
    pub const MAP_16_TYPE: u8 = 0xde;
    pub const MAP_32_TYPE: u8 = 0xdf;
}

impl WriteTo for Vec<(Value<'_>, Value<'_>)> {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let map_length = self.len();

        match map_length {
            0..=15 => buffer.write_all(&[Map::FIXMAP + map_length as u8])?,
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

impl<'a> ReadFrom<'a> for Vec<(Value<'a>, Value<'a>)> {
    #[inline(always)]
    fn read_from<U: AsRef<[u8]> + 'a>(packet_type: u8, reader: &'a mut Reader<U>) -> Result<Self> {
        let vec_length = match packet_type {
            _ if ((Map::FIXMAP..(Map::FIXMAP + 0x0f)).contains(&packet_type)) => {
                packet_type as usize - Map::FIXMAP as usize
            }
            Map::MAP_16_TYPE => {
                let buffer = reader.pull(2)?;

                u16::from_be_bytes([buffer[0], buffer[1]]) as usize
            }
            Map::MAP_32_TYPE => {
                let buffer = reader.pull(4)?;

                u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize
            }
            _ => return Ok(Vec::new()),
        };

        let mut vec = Vec::with_capacity(vec_length);

        for _ in 0..vec_length {
            unsafe {
                let reader_ptr0 = reader as *mut Reader<U> as *mut Reader<U>;
                let reader_ptr = &mut *reader_ptr0;

                let key = { reader_ptr.pull_value()? };

                let reader_ptr1 = &mut *reader_ptr0;

                let value = reader_ptr1.pull_value()?;

                vec.push((key, value));
            }
        }

        Ok(vec)
    }
}
