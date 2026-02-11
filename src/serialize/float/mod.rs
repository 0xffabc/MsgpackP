use crate::constants::Families;
use crate::serialize::WriteTo;
use anyhow::Result;
use std::io::Write;

impl WriteTo for f32 {
    #[inline(always)]
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::FLOAT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl WriteTo for f64 {
    #[inline(always)]
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[
            Families::FLOAT64,
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
        ])?;

        Ok(())
    }
}
