use anyhow::Result;

use crate::{
    constants::Families,
    msgpack::{ReadFrom, array::Array, ext::Extension},
    value::Value,
};

pub struct Reader<R: AsRef<[u8]>> {
    read: R,
    index: usize,
}

impl<R: AsRef<[u8]>> Reader<R> {
    #[inline(always)]
    /**
     * @name new
     * @description
     *
     * Creates a packet reader.
     *
     * Remember: It's always better to pass a slice: not a Vec<u8> or Box<[u8]>.
     */
    pub fn new(read: R) -> Self {
        Reader { read, index: 0 }
    }

    #[inline]
    /**
     * @name pull_value
     * @description
     *
     * Reads a certain value from inner byte slice.
     */
    pub fn pull_value<'a>(&'a mut self) -> Result<Value<'a>> {
        let packet_type = self.pull(1)[0];

        Ok(match packet_type {
            /* Array */
            0x90..0x9f => Value::array(Box::<[Value]>::read_from(packet_type, self)?),
            Array::ARRAY_16_TYPE | Array::ARRAY_32_TYPE => {
                Value::array(Box::<[Value]>::read_from(packet_type, self)?)
            }

            /* String */
            0xa0..=0xbf => Value::str(self.pull_string(packet_type)),
            Families::STR8 | Families::STR16 | Families::STR32 => {
                Value::str(self.pull_string(packet_type))
            }

            /* Positive fixint */
            0x00..0x7f => Value::u8(u8::read_from(packet_type, self)?),

            /* Negative fixiunt */
            0xe0..=0xff => Value::i8(i8::read_from(packet_type, self)?),

            /* Map */
            0x80..0x8f => Value::map(Box::<[(Value, Value)]>::read_from(packet_type, self)?),
            Families::MAP16 | Families::MAP32 => {
                Value::map(Box::<[(Value, Value)]>::read_from(packet_type, self)?)
            }

            /* Float */
            Families::FLOAT32 => Value::f32(ordered_float::OrderedFloat(f32::read_from(
                packet_type,
                self,
            )?)),
            Families::FLOAT64 => Value::f64(ordered_float::OrderedFloat(f64::read_from(
                packet_type,
                self,
            )?)),

            /* Integer */
            Families::UINT8 => Value::u8(u8::read_from(packet_type, self)?),
            Families::UINT16 => Value::u16(u16::read_from(packet_type, self)?),
            Families::UINT32 => Value::u32(u32::read_from(packet_type, self)?),
            Families::UINT64 => Value::u64(u64::read_from(packet_type, self)?),
            Families::INT8 => Value::i8(i8::read_from(packet_type, self)?),
            Families::INT16 => Value::i16(i16::read_from(packet_type, self)?),
            Families::INT32 => Value::i32(i32::read_from(packet_type, self)?),
            Families::INT64 => Value::i64(i64::read_from(packet_type, self)?),

            Families::NIL | Families::RESERVED => Value::Nil,
            Families::FALSE | Families::TRUE => Value::bool(bool::read_from(packet_type, self)?),
            Families::BIN8 | Families::BIN16 | Families::BIN32 => Value::bin(
                Vec::<u8>::read_from(packet_type, self)?
                    .iter()
                    .map(|&byte| byte)
                    .collect::<Vec<_>>(),
            ),

            /* Ext */
            Families::FIXEXT1
            | Families::FIXEXT2
            | Families::FIXEXT4
            | Families::FIXEXT8
            | Families::FIXEXT16
            | Families::EXT8
            | Families::EXT16
            | Families::EXT32 => Value::extension(Extension::read_from(packet_type, self)?),

            /* Reserved */
            127..=191 => Value::Nil,
        })
    }

    /**
     * @name pull_string
     * @description
     *
     * Pulls a &'a str from underlying buffer
     * Guaranteed to never crash
     */
    #[inline]
    pub fn pull_string<'a>(&'a mut self, strtype: u8) -> &'a str {
        let len = match strtype {
            /*
             * Fixed strings start from 0xa0 and end at 0xbf
             *
             * https://github.com/msgpack/msgpack/blob/master/spec.md#:~:text=101xxxxx-,0xa0%20%2D%200xbf,-nil
             */
            0xa0..0xbf => (strtype - 0xa0) as usize,

            /*
             * 1 byte per u8
             */
            Families::STR8 => self.pull(1)[0] as usize,

            /*
             * 2 bytes per u16
             */
            Families::STR16 => {
                let bytes = self.pull(2);

                if bytes.len() != 2 {
                    return "";
                }

                u16::from_be_bytes([bytes[0], bytes[1]]) as usize
            }

            /*
             * 4 bytes per u32
             */
            Families::STR32 => {
                let bytes = self.pull(4);

                if bytes.len() != 4 {
                    return "";
                }

                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
            }

            /*
             * Likely an error, corrupt the whole message by reading one byte
             *
             * So we can discard it on the application level afterwards.
             *
             * 'A correct client doesn’t make protocol errors' ~ Aspect
             */
            _ => 1usize,
        };

        std::str::from_utf8(self.pull(len)).unwrap_or("")
    }

    /**
     * @name pull
     * @description
     *
     * Pulls a slice from the underlying buffer.
     * Safe as long as everything is aligned properly.
     */
    #[inline(always)]
    pub fn pull<'a>(&'a mut self, len: usize) -> &'a [u8] {
        let bytes = self.read.as_ref();

        unsafe {
            /*
             * Safety: bytes slice is aligned, self.index is aligned too
             *
             * We don't pull them from C++ (thank god)
             *
             * Data is non-null and valid.
             */

            let ptr = bytes.as_ptr().add(self.index);

            self.index += len;

            std::slice::from_raw_parts(ptr, len)
        }
    }
}
