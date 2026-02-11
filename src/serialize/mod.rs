use anyhow::Result;
use std::io::Write;

pub mod array;
pub mod bin;
pub mod boolean;
pub mod ext;
pub mod float;
pub mod integer;
pub mod map;
pub mod null;
pub mod string;

pub mod value;

pub trait WriteTo {
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::value::Value;

    #[test]
    fn test_write_to() {
        let value = Value::U8(42);
        let mut buffer = Vec::new();
        value.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, &[0x2a]);
    }

    #[test]
    fn test_arrays() {
        let mut buffer = Vec::new();

        let packet = vec![Value::U8(1), Value::Str("hai".to_string())];

        packet.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, &[0x92, 0x01, 0xa3, b'h', b'a', b'i']);
    }

    #[test]
    fn test_typical_moomoo_packet() {
        let mut buffer = Vec::new();

        let packet = vec![
            Value::Str("sp".to_string()),
            Value::Array(vec![Value::Map(vec![(
                Value::Str("name".to_string()),
                Value::Str("0xffabc".to_string()),
            )])]),
        ];

        packet.write_to(&mut buffer).unwrap();
        assert_eq!(
            buffer,
            &[
                0x92, 0xa2, b's', b'p', 0x91, 0x81, 0xa4, b'n', b'a', b'm', b'e', 0xa7, b'0', b'x',
                b'f', b'f', b'a', b'b', b'c'
            ]
        );
    }
}
