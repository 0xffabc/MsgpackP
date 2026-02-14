use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl WriteTo for f32 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::FLOAT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for f32 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(4)?;

        Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl WriteTo for f64 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
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

impl<'a> ReadFrom<'a> for f64 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Reader<T>) -> Result<Self> {
        let bytes = reader.pull(8)?;

        Ok(f64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}
