use crate::constants::Families;
use crate::serialize::WriteTo;
use anyhow::Result;
use std::io::Write;

impl WriteTo for u8 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match self {
            0..=0x7f => writer.write_all(&[*self])?,
            _ => writer.write_all(&[Families::UINT8, *self])?,
        }

        Ok(())
    }
}

impl WriteTo for u16 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::UINT16, bytes[0], bytes[1]])?;

        Ok(())
    }
}

impl WriteTo for u32 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::UINT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl WriteTo for u64 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[
            Families::UINT64,
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

impl WriteTo for i8 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match self {
            -32..=-1 => {
                let positive: u8 = u8::try_from(-self)?;

                writer.write_all(&[positive])?;
            }
            _ => writer.write_all(&[Families::INT8, *self as u8])?,
        }

        Ok(())
    }
}

impl WriteTo for i16 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::INT16, bytes[0], bytes[1]])?;

        Ok(())
    }
}

impl WriteTo for i32 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[Families::INT32, bytes[0], bytes[1], bytes[2], bytes[3]])?;

        Ok(())
    }
}

impl WriteTo for i64 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let bytes = self.to_be_bytes();
        writer.write_all(&[
            Families::INT64,
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
