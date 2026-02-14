use anyhow::Result;
use std::io::Write;

use crate::reader::Reader;

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

pub trait ReadFrom<'a> {
    fn read_from<T: AsRef<[u8]> + 'a>(packet_type: u8, reader: &'a mut Reader<T>) -> Result<Self>
    where
        Self: 'a + Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;
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

        let packet = vec![Value::U8(1), Value::Str("hai")];

        packet.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, &[0x92, 0x01, 0xa3, b'h', b'a', b'i']);
    }

    #[test]
    fn test_typical_moomoo_packet() {
        let mut buffer = Vec::new();

        let packet = vec![
            Value::Str("sp"),
            Value::Array(vec![Value::Map(vec![(
                Value::Str("name"),
                Value::Str("0xffabc"),
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

        let mut reader = Reader::new(&packet);
        let val = reader.pull_value().unwrap();

        assert_eq!(
            val,
            Value::Array(vec![
                Value::Str("ch"),
                Value::Array(vec![
                    Value::Str("Hello"),
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

        let mut reader = Reader::new(&example);
        let val = reader.pull_value().unwrap();

        assert_eq!(
            val,
            Value::Map(vec![
                (Value::str("int"), Value::u8(1)),
                (Value::str("float"), Value::f64(OrderedFloat(0.5))),
                (Value::str("boolean"), Value::bool(true)),
                (Value::str("null"), Value::Nil),
                (Value::str("string"), Value::str("foo bar")),
                (
                    Value::str("array"),
                    Value::Array(vec![Value::str("foo"), Value::str("bar")])
                ),
                (
                    Value::str("object"),
                    Value::Map(vec![
                        (Value::str("foo"), Value::u8(1)),
                        (Value::str("baz"), Value::f64(OrderedFloat(0.5))),
                    ])
                )
            ])
        );
    }
}
