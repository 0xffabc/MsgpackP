use msgpackp::{msgpack::WriteTo, value::Value};

fn main() {
    let packet = vec![
        Value::Str("sp"),
        Value::Array(
            vec![Value::Map(
                vec![(Value::Str("name"), Value::Str("0xffabc"))].into_boxed_slice(),
            )]
            .into_boxed_slice(),
        ),
    ]
    .into_boxed_slice();

    let mut buffer = Vec::new();

    packet.write_to(&mut buffer).unwrap();
}
