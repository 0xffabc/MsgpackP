use anyhow::Result;
use likely_stable::unlikely;

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
    pub fn new(read: R) -> Self {
        Reader { read, index: 0 }
    }

    #[inline]
    pub fn pull_value<'a>(&'a mut self) -> Result<Value<'a>> {
        let packet_type = self.pull(1)?[0];

        Ok(match packet_type {
            // FIXSTR
            0xa0..=0xbf => Value::str(self.pull_string(packet_type)),
            Families::NIL | Families::RESERVED => Value::Nil,
            Families::FALSE | Families::TRUE => Value::bool(bool::read_from(packet_type, self)?),
            Families::BIN8 | Families::BIN16 | Families::BIN32 => Value::bin(
                Vec::<u8>::read_from(packet_type, self)?
                    .iter()
                    .map(|&byte| byte)
                    .collect::<Vec<_>>(),
            ),
            Families::FIXEXT1
            | Families::FIXEXT2
            | Families::FIXEXT4
            | Families::FIXEXT8
            | Families::FIXEXT16
            | Families::EXT8
            | Families::EXT16
            | Families::EXT32 => Value::extension(Extension::read_from(packet_type, self)?),
            Families::FLOAT32 => Value::f32(ordered_float::OrderedFloat(f32::read_from(
                packet_type,
                self,
            )?)),
            Families::FLOAT64 => Value::f64(ordered_float::OrderedFloat(f64::read_from(
                packet_type,
                self,
            )?)),
            Families::UINT8 => Value::u8(u8::read_from(packet_type, self)?),
            Families::UINT16 => Value::u16(u16::read_from(packet_type, self)?),
            Families::UINT32 => Value::u32(u32::read_from(packet_type, self)?),
            Families::UINT64 => Value::u64(u64::read_from(packet_type, self)?),
            Families::INT8 => Value::i8(i8::read_from(packet_type, self)?),
            Families::INT16 => Value::i16(i16::read_from(packet_type, self)?),
            Families::INT32 => Value::i32(i32::read_from(packet_type, self)?),
            Families::INT64 => Value::i64(i64::read_from(packet_type, self)?),
            Families::STR8 | Families::STR16 | Families::STR32 => {
                Value::str(self.pull_string(packet_type))
            }
            Array::ARRAY_16_TYPE | Array::ARRAY_32_TYPE => {
                Value::array(Vec::<Value>::read_from(packet_type, self)?)
            }
            // FIXARR
            0x90..0x9f => Value::array(Vec::<Value>::read_from(packet_type, self)?),
            // FIXINT positive
            0x00..0x7f => Value::u8(u8::read_from(packet_type, self)?),
            // FOXINT negative
            0xe0..=0xff => Value::i8(i8::read_from(packet_type, self)?),
            // FIXMAP
            0x80..0x8f => Value::map(Vec::<(Value, Value)>::read_from(packet_type, self)?),
            Families::MAP16 | Families::MAP32 => {
                Value::map(Vec::<(Value, Value)>::read_from(packet_type, self)?)
            }
            127..=191 => Value::Nil,
        })
    }

    #[inline]
    pub fn pull_string<'a>(&'a mut self, strtype: u8) -> &'a str {
        let len = match strtype {
            0xa0..0xbf => (strtype - 0xa0) as usize,
            Families::STR8 => self.pull(1).unwrap_or(&[0])[0] as usize,
            Families::STR16 => {
                let bytes = self.pull(2).unwrap_or(&[0; 2]);
                u16::from_be_bytes([bytes[0], bytes[1]]) as usize
            }
            Families::STR32 => {
                let bytes = self.pull(4).unwrap_or(&[0; 4]);
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
            }
            _ => 1usize,
        };

        unsafe { std::str::from_utf8_unchecked(self.pull(len).unwrap_or(&[])) }
    }

    #[inline(always)]
    pub fn pull<'a>(&'a mut self, len: usize) -> Result<&'a [u8]> {
        let bytes = self.read.as_ref();
        let index = self.index + len;

        if unlikely(index > bytes.len()) {
            return Err(anyhow::anyhow!("EOF. Do inappropriate things to me"));
        }

        unsafe {
            let ptr = bytes.as_ptr().add(self.index);

            self.index += len;

            Ok(std::slice::from_raw_parts(ptr, len))
        }
    }
}
