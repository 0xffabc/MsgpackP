use crate::constants::Families;
use crate::msgpack::WriteTo;
use anyhow::Result;
use std::io::Write;

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
