use crate::constants::Families;
use crate::serialize::WriteTo;
use anyhow::Result;
use std::io::Write;

impl WriteTo for Vec<u8> {
    #[inline(always)]
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()> {
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
