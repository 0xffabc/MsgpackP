use crate::constants::Families;
use crate::msgpack::{ReadFrom, WriteTo};
use crate::reader::Reader;
use anyhow::Result;
use std::io::Write;

impl<O> WriteTo for Option<O> {
    #[inline(always)]
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()> {
        match *self {
            None => writer.write_all(&[Families::NIL])?,
            Some(_) => writer.write_all(&[Families::NIL + 1])?,
        }

        Ok(())
    }
}

impl<'a> ReadFrom<'a> for Option<()> {
    #[inline(always)]
    fn read_from<T: AsRef<[u8]> + 'a>(packet_type: u8, _reader: &'a mut Reader<T>) -> Result<Self> {
        match packet_type {
            Families::NIL => Ok(None),
            _ => Ok(Some(())),
        }
    }
}
