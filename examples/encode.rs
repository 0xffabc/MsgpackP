use msgpackp::{msgpack::WriteTo, value::Value};

fn main() {
    let packet = vec![
        Value::Str("sp"),
        Value::Array(vec![Value::Map(vec![(
            Value::Str("name"),
            Value::Str("0xffabc"),
        )])]),
    ];

    let mut buffer = Vec::new();

    packet.write_to(&mut buffer).unwrap();
}
