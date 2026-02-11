use crate::core::conversions::{Serialized, Value};

impl From<i8> for Serialized {
    fn from(value: i8) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<i16> for Serialized {
    fn from(value: i16) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<i32> for Serialized {
    fn from(value: i32) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<i64> for Serialized {
    fn from(value: i64) -> Self {
        Serialized(value.to_be_bytes().to_vec())
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::IntegerI8(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::IntegerI16(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::IntegerI32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::IntegerI64(value)
    }
}
