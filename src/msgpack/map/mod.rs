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

impl WriteTo for Box<[(Value<'_>, Value<'_>)]> {
    #[inline(always)]
    fn write_to<U: Write>(&self, buffer: &mut U) -> Result<()> {
        let map_length = self.len();

        match map_length {
            /*
             * Fixmap size is 15
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#:~:text=1000xxxx-,0x80%20%2D%200x8f,-fixarray
             */
            0..=15 => buffer.write_all(&[Map::FIXMAP + map_length as u8])?,

            /*
             * map 16 stores a map whose length is upto (2^16)-1 elements
             * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
             * |  0xde  |YYYYYYYY|YYYYYYYY|   N*2 objects   |
             * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
             */
            16..=65534 => {
                buffer.write_all(&[Map::MAP_16_TYPE])?;
                buffer.write_all(&map_length.to_be_bytes())?
            }

            /*
             * map 32 stores a map whose length is upto (2^32)-1 elements
             * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
             * |  0xdf  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|   N*2 objects   |
             * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
             */
            65535..4294967295 => {
                buffer.write_all(&[Map::MAP_32_TYPE])?;
                buffer.write_all(&map_length.to_be_bytes())?
            }

            /*
             * Msgpack doesn't support map64.
             *
             * However you can make an extension if you need it.
             */
            _ => {
                return Err(anyhow::anyhow!(
                    "MAP64 is not supported in msgpack. Consider making an extension instead."
                ));
            }
        }

        for (key, value) in self {
            key.write_to(buffer)?;
            value.write_to(buffer)?;
        }

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for Box<[(Value<'a>, Value<'a>)]> {
    #[inline(always)]
    fn read_from<U: AsRef<[u8]> + 'a>(packet_type: u8, reader: &'a mut Reader<U>) -> Result<Self> {
        let map_length = match packet_type {
            /*
             * Fixmap ranges from 0x80 to 0x8f:
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#map-format-family:~:text=1000xxxx-,0x80%20%2D%200x8f,-fixarray
             */
            0x80..0x8f => packet_type as usize - Map::FIXMAP as usize,

            /*
             * map 16 stores a map whose length is upto (2^16)-1 elements
             * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
             * |  0xde  |YYYYYYYY|YYYYYYYY|   N*2 objects   |
             * +--------+--------+--------+~~~~~~~~~~~~~~~~~+
             */
            Map::MAP_16_TYPE => {
                let buffer = reader.pull(2)?;

                u16::from_be_bytes([buffer[0], buffer[1]]) as usize
            }

            /*
             * map 32 stores a map whose length is upto (2^32)-1 elements
             * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
             * |  0xdf  |ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|ZZZZZZZZ|   N*2 objects   |
             * +--------+--------+--------+--------+--------+~~~~~~~~~~~~~~~~~+
             */
            Map::MAP_32_TYPE => {
                let buffer = reader.pull(4)?;

                u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize
            }

            /*
             * Do not read.
             */
            _ => {
                return Err(anyhow::anyhow!(
                    "Failed to read MAP64. The serializer knows something I don't possess."
                ));
            }
        };

        /*
         * Prevent people from allocating 4GB
         */
        if map_length > 100usize {
            return Err(anyhow::anyhow!(
                "A map size of 100??? This is VERY unrealistic for moomoo.io"
            ));
        }

        /*
         * Note: DO NOT USE with_capacity!
         *
         * Preemptive allocations slow down everything **4 times**
         */

        let mut uninit = Box::<[(Value, Value)]>::new_uninit_slice(map_length);

        let reader_ptr0 = reader as *mut Reader<U>;

        unsafe {
            for i in 0..map_length {
                let key = (&mut *reader_ptr0).pull_value()?;
                let value = (&mut *reader_ptr0).pull_value()?;

                uninit[i].write((key, value));
            }

            Ok(uninit.assume_init())
        }
    }
}
