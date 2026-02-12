use crate::constants::Families;
use crate::serialize::{ReadFrom, WriteTo};
use anyhow::Result;
use std::io::{Cursor, Write};

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

impl ReadFrom for Option<()> {
    #[inline(always)]
    fn read_from(packet_type: u8, _reader: &mut Cursor<Vec<u8>>) -> Self {
        match packet_type {
            Families::NIL => None,
            _ => Some(()),
        }
    }
}
