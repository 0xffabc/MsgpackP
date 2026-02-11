use crate::core::conversions::{Serialized, Value};

impl From<String> for Serialized {
    fn from(value: String) -> Self {
        Serialized(value.into_bytes())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}
