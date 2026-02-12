use anyhow::Result;
use std::io::{Cursor, Write};

pub mod array;
pub mod bin;
pub mod boolean;
pub mod ext;
pub mod float;
pub mod integer;
pub mod map;
pub mod null;
pub mod string;

pub trait WriteTo {
    fn write_to<U: Write>(&self, writer: &mut U) -> Result<()>;
}

pub trait ReadFrom {
    fn read_from(packet_type: u8, reader: &mut Cursor<Vec<u8>>) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{Value, read_value_from_cursor};

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

    #[test]
    fn test_deserialize() {
        let packet = vec![
            146, 162, 99, 104, 147, 165, 72, 101, 108, 108, 111, 1, 203, 63, 244, 204, 204, 204,
            204, 204, 205,
        ];

        let val = read_value_from_cursor(&mut Cursor::new(packet));

        assert_eq!(
            val,
            Value::Array(vec![
                Value::Str("ch".to_string()),
                Value::Array(vec![
                    Value::Str("Hello".to_string()),
                    Value::U8(1),
                    Value::F64(1.3)
                ])
            ])
        );
    }
}
