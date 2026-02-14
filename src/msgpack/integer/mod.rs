use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Read, Write};

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

impl ReadFrom for u8 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, _reader: &mut Cursor<T>) -> Result<Self> {
        Ok(packet_type)
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

impl ReadFrom for u16 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut byte = [0u8; 2];

        reader.read_exact(&mut byte)?;

        Ok(u16::from_be_bytes(byte))
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

impl ReadFrom for u32 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut byte = [0u8; 4];

        reader.read_exact(&mut byte)?;

        Ok(u32::from_be_bytes(byte))
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

impl ReadFrom for u64 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut byte = [0u8; 8];

        reader.read_exact(&mut byte)?;

        Ok(u64::from_be_bytes(byte))
    }
}

impl WriteTo for i8 {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match self {
            -32..=-1 => {
                let positive: u8 = u8::try_from(-self)?;

                writer.write_all(&[0xe0 + positive])?;
            }
            _ => writer.write_all(&[Families::INT8, *self as u8])?,
        }

        Ok(())
    }
}

impl ReadFrom for i8 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        if packet_type >= 0xe0 {
            return Ok(-((!packet_type as i8).wrapping_add(1)));
        }

        let mut byte = [0u8; 1];

        reader.read_exact(&mut byte)?;

        Ok(i8::from_be_bytes(byte))
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

impl ReadFrom for i16 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut bytes = [0u8; 2];

        reader.read_exact(&mut bytes)?;

        Ok(i16::from_be_bytes(bytes))
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

impl ReadFrom for i32 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut bytes = [0u8; 4];

        reader.read_exact(&mut bytes)?;

        Ok(i32::from_be_bytes(bytes))
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

impl ReadFrom for i64 {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(_packet_type: u8, reader: &mut Cursor<T>) -> Result<Self> {
        let mut bytes = [0u8; 8];

        reader.read_exact(&mut bytes)?;

        Ok(i64::from_be_bytes(bytes))
    }
}
