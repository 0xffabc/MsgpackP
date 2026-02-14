use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl WriteTo for [u8; 1] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT1])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for [u8; 1] {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, _reader: &mut Reader<T>) -> Result<Self> {
        Ok([packet_type])
    }
}

impl WriteTo for [u8; 2] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT2])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for [u8; 2] {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(2)?;
        Ok([bytes[0], bytes[1]])
    }
}

impl WriteTo for [u8; 4] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT4])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for [u8; 4] {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(4)?;
        Ok([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

impl WriteTo for [u8; 8] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT8])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for [u8; 8] {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(8)?;
        Ok([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ])
    }
}

impl WriteTo for [u8; 16] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT16])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for [u8; 16] {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(16)?;
        Ok([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension {
    type_: u8,
    data: Vec<u8>,
}

impl Extension {
    pub fn new(type_: u8, data: Vec<u8>) -> Self {
        Extension { type_, data }
    }
}

impl WriteTo for Extension {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let data_len = self.data.len();

        match data_len {
            0..=255 => {
                writer.write_all(&[Families::EXT8])?;
                writer.write_all(&(data_len as u8).to_be_bytes())?;
            }
            256..=65535 => {
                writer.write_all(&[Families::EXT16])?;
                writer.write_all(&(data_len as u16).to_be_bytes())?;
            }
            _ => {
                writer.write_all(&[Families::EXT32])?;
                writer.write_all(&(data_len as u32).to_be_bytes())?;
            }
        }
        writer.write_all(&self.type_.to_be_bytes())?;
        writer.write_all(&self.data)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for Extension {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(4)?;
        let data_len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let data = reader.pull(data_len)?.to_vec();

        Ok(Extension {
            type_: packet_type,
            data,
        })
    }
}
