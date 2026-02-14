use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Read, Write};

impl WriteTo for f32 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::FLOAT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl ReadFrom for f32 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut bytes = [0u8; 4];

        reader.read_exact(&mut bytes)?;

        Ok(f32::from_be_bytes(bytes))
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

impl ReadFrom for f64 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut bytes = [0u8; 8];

        reader.read_exact(&mut bytes)?;

        Ok(f64::from_be_bytes(bytes))
    }
}
