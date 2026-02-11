use crate::constants::Families;
use crate::serialize::WriteTo;
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

impl WriteTo for [u8; 2] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT2])?;
        writer.write_all(self)?;

        Ok(())
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

impl WriteTo for [u8; 8] {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        writer.write_all(&[Families::FIXEXT8])?;
        writer.write_all(self)?;

        Ok(())
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
