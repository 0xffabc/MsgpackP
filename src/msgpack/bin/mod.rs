use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl WriteTo for Vec<u8> {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let len = self.len();

        if len <= 31 {
            writer.write_all(&[Families::BIN8])?;
            writer.write_all(&[len as u8])?;
        } else if len <= 255 {
            writer.write_all(&[Families::BIN16])?;
            writer.write_all(&(len as u16).to_be_bytes())?;
        } else if len <= 65535 {
            writer.write_all(&[Families::BIN32])?;
            writer.write_all(&(len as u32).to_be_bytes())?;
        }

        writer.write_all(self)?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for Vec<u8> {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let len = match packet_type {
            Families::BIN8 => reader.pull(1)?[0] as usize,
            Families::BIN16 => {
                let buf = reader.pull(2)?;
                u16::from_be_bytes([buf[0], buf[1]]) as usize
            }
            Families::BIN32 => {
                let buf = reader.pull(4)?;
                u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize
            }
            _ => 0,
        };

        let data = reader.pull(len)?;

        Ok(data.to_vec())
    }
}
