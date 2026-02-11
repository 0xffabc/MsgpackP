use crate::constants::Families;
use crate::serialize::WriteTo;
use anyhow::Result;
use std::io::Write;

impl<O> WriteTo for Option<O> {
    #[inline(always)]
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()> {
        match *self {
            None => writer.write_all(&[Families::NIL])?,
            Some(_) => writer.write_all(&[Families::NIL + 1])?,
        }

        Ok(())
    }
}
