use crate::constants::Families;
use crate::serialize::WriteTo;
use anyhow::Result;
use std::io::Write;

impl WriteTo for bool {
    #[inline(always)]
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()> {
        let byte = if *self {
            Families::TRUE
        } else {
            Families::FALSE
        };

        writer.write_all(&[byte])?;

        Ok(())
    }
}
