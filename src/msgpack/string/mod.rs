use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Read, Write};

impl WriteTo for String {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let len = self.len();

        if len <= 31 {
            writer.write_all(&[Families::FIXSTR + len as u8])?;
        } else if len <= 255 {
            writer.write_all(&[Families::STR8])?;
            writer.write_all(&[len as u8])?;
        } else if len <= 65535 {
            writer.write_all(&[Families::STR16])?;
            writer.write_all(&(len as u16).to_be_bytes())?;
        } else {
            writer.write_all(&[Families::STR32])?;
            writer.write_all(&(len as u32).to_be_bytes())?;
        }

        writer.write_all(self.as_bytes())?;

        Ok(())
    }
}

impl ReadFrom for String {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        match packet_type {
            _ if (Families::FIXSTR..=Families::FIXSTR + 0x1f).contains(&packet_type) => {
                let len = packet_type - 0xa0;

                let mut buf = vec![0; len as usize];

                reader.read_exact(&mut buf)?;

                Ok(String::from_utf8(buf)?)
            }
            Families::STR8 => {
                let mut len_buf = [0; 1];

                reader.read_exact(&mut len_buf)?;

                let len = len_buf[0];
                let mut buf = vec![0; len as usize];

                reader.read_exact(&mut buf)?;

                Ok(String::from_utf8(buf)?)
            }
            Families::STR16 => {
                let mut len_buf = [0; 2];

                reader.read_exact(&mut len_buf)?;

                let len = u16::from_be_bytes(len_buf);
                let mut buf = vec![0; len as usize];

                reader.read_exact(&mut buf)?;

                Ok(String::from_utf8(buf)?)
            }
            Families::STR32 => {
                let mut len_buf = [0; 4];

                reader.read_exact(&mut len_buf)?;

                let len = u32::from_be_bytes(len_buf);
                let mut buf = vec![0; len as usize];

                reader.read_exact(&mut buf)?;

                Ok(String::from_utf8(buf)?)
            }
            _ => Ok(String::new()),
        }
    }
}
