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
    use ordered_float::OrderedFloat;

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
                    Value::F64(OrderedFloat(1.3)),
                ])
            ])
        );
    }

    #[test]
    fn test_hashmap() {
        let example = vec![
            135, 163, 105, 110, 116, 1, 165, 102, 108, 111, 97, 116, 203, 63, 224, 0, 0, 0, 0, 0,
            0, 167, 98, 111, 111, 108, 101, 97, 110, 195, 164, 110, 117, 108, 108, 192, 166, 115,
            116, 114, 105, 110, 103, 167, 102, 111, 111, 32, 98, 97, 114, 165, 97, 114, 114, 97,
            121, 146, 163, 102, 111, 111, 163, 98, 97, 114, 166, 111, 98, 106, 101, 99, 116, 130,
            163, 102, 111, 111, 1, 163, 98, 97, 122, 203, 63, 224, 0, 0, 0, 0, 0, 0,
        ];

        let val: Value = read_value_from_cursor(&mut Cursor::new(example));

        assert_eq!(
            val,
            Value::Map(vec![
                (Value::Str("int".to_string()), Value::U8(1)),
                (
                    Value::Str("float".to_string()),
                    Value::F64(OrderedFloat(0.5))
                ),
                (Value::Str("boolean".to_string()), Value::Bool(true)),
                (Value::Str("null".to_string()), Value::Nil),
                (
                    Value::Str("string".to_string()),
                    Value::Str("foo bar".to_string())
                ),
                (
                    Value::Str("array".to_string()),
                    Value::Array(vec![
                        Value::Str("foo".to_string()),
                        Value::Str("bar".to_string())
                    ])
                ),
                (
                    Value::Str("object".to_string()),
                    Value::Map(vec![
                        (Value::Str("foo".to_string()), Value::U8(1)),
                        (Value::Str("baz".to_string()), Value::F64(OrderedFloat(0.5))),
                    ])
                )
            ])
        );
    }
}
