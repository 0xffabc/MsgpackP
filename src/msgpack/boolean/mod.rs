use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Write};

impl WriteTo for bool {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        let byte = if *self {
            Families::TRUE
        } else {
            Families::FALSE
        };

        writer.write_all(&[byte])?;

        Ok(())
    }
}

impl ReadFrom for bool {
    #[inline(always)]
    fn read_from(packet_type: u8, _reader: &mut Cursor<Box<[u8]>>) -> Result<Self> {
        Ok(packet_type == Families::TRUE)
    }
}
