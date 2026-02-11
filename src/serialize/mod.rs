use anyhow::Result;
use std::io::Write;

pub mod bin;
pub mod boolean;
pub mod ext;
pub mod float;
pub mod integer;
pub mod null;
pub mod string;

pub trait WriteTo {
    fn write_to<T, U: Write>(&self, writer: &mut U) -> Result<()>;
}
