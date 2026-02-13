use crate::constants::Families;
use crate::serialize::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Read, Write};

impl WriteTo for [u8; 1] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT1])?;
        writer.write_all(self)?;

        Ok(())
    }
}

impl ReadFrom for [u8; 1] {
    #[inline(always)]
    fn read_from(packet_type: u8, _reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
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

impl ReadFrom for [u8; 2] {
    #[inline(always)]
    fn read_from(_packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut bytes = [0u8; 2];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
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

impl ReadFrom for [u8; 4] {
    #[inline(always)]
    fn read_from(_packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
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

impl ReadFrom for [u8; 8] {
    #[inline(always)]
    fn read_from(_packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
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

impl ReadFrom for [u8; 16] {
    #[inline(always)]
    fn read_from(_packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension {
    type_: u8,
    data: Vec<u8>,
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

impl ReadFrom for Extension {
    #[inline(always)]
    fn read_from(packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let mut data_len_buffer = [0u8; 4];

        reader.read_exact(&mut data_len_buffer)?;

        let data_len = u32::from_be_bytes(data_len_buffer) as usize;

        let mut data = vec![0u8; data_len];

        reader.read_exact(&mut data)?;

        Ok(Extension {
            type_: packet_type,
            data,
        })
    }
}
