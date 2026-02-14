use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Read, Write};

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

impl ReadFrom for Vec<u8> {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let len = match packet_type {
            Families::BIN8 => {
                let mut len_bytes = [0; 1];
                reader.read_exact(&mut len_bytes)?;
                len_bytes[0] as usize
            }
            Families::BIN16 => {
                let mut len_bytes = [0; 2];
                reader.read_exact(&mut len_bytes)?;
                u16::from_be_bytes(len_bytes) as usize
            }
            Families::BIN32 => {
                let mut len_bytes = [0; 4];
                reader.read_exact(&mut len_bytes)?;
                u32::from_be_bytes(len_bytes) as usize
            }
            _ => 0,
        };

        let mut data = Vec::with_capacity(len);

        reader.read_exact(&mut data)?;

        Ok(data)
    }
}
