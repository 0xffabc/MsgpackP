use crate::core::conversions::{Serialized, Value};

impl From<u8> for Serialized {
    fn from(value: u8) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<u16> for Serialized {
    fn from(value: u16) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<u32> for Serialized {
    fn from(value: u32) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<u64> for Serialized {
    fn from(value: u64) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::IntegerU8(value)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::IntegerU16(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::IntegerU32(value)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::IntegerU64(value)
    }
}
