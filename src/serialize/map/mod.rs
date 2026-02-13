use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
};

use anyhow::Result;

use crate::{
    serialize::{ReadFrom, WriteTo},
    value::{Value, read_value_from_cursor},
};

struct Map();

impl Map {
    pub const FIXMAP: u8 = 0x80;
    pub const MAP_16_TYPE: u8 = 0xde;
    pub const MAP_32_TYPE: u8 = 0xdf;
}

impl WriteTo for Vec<(Value, Value)> {
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

impl ReadFrom for HashMap<Value, Value> {
    fn read_from(packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut map = HashMap::new();
        let map_length = match packet_type {
            _ if ((Map::FIXMAP..(Map::FIXMAP + 0x0f)).contains(&packet_type)) => {
                packet_type as usize - Map::FIXMAP as usize
            }
            Map::MAP_16_TYPE => {
                let mut buffer = [0; 2];
                reader.read_exact(&mut buffer)?;

                u16::from_be_bytes(buffer) as usize
            }
            Map::MAP_32_TYPE => {
                let mut buffer = [0; 4];
                reader.read_exact(&mut buffer)?;

                u32::from_be_bytes(buffer) as usize
            }
            _ => return Ok(HashMap::new()),
        };

        for _ in 0..map_length {
            let key = read_value_from_cursor(reader)?;
            let value = read_value_from_cursor(reader)?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl ReadFrom for Vec<(Value, Value)> {
    fn read_from(packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut vec = Vec::new();
        let vec_length = match packet_type {
            _ if ((Map::FIXMAP..(Map::FIXMAP + 0x0f)).contains(&packet_type)) => {
                packet_type as usize - Map::FIXMAP as usize
            }
            Map::MAP_16_TYPE => {
                let mut buffer = [0; 2];
                reader.read_exact(&mut buffer)?;

                u16::from_be_bytes(buffer) as usize
            }
            Map::MAP_32_TYPE => {
                let mut buffer = [0; 4];
                reader.read_exact(&mut buffer)?;

                u32::from_be_bytes(buffer) as usize
            }
            _ => return Ok(Vec::new()),
        };

        for _ in 0..vec_length {
            let key = read_value_from_cursor(reader)?;
            let value = read_value_from_cursor(reader)?;

            vec.push((key, value));
        }

        Ok(vec)
    }
}
