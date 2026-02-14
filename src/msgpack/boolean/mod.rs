use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

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

impl<'a> ReadFrom<'a> for bool {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]>>(packet_type: u8, _reader: &mut Reader<T>) -> Result<Self> {
        Ok(packet_type == Families::TRUE)
    }
}
