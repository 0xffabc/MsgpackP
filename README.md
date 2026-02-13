# Currently undergoing major changes

> [!WARN]
> MsgpackP is no longer a simple .js script, however the .js version will remain in the repository root as a relic of the time

# Building

```
cargo build
```

# Public APIs

Serialize with

```rust
let mut buffer = Vec::new();

let packet = vec![
    Value::Str("sp".to_string()),
    Value::Array(vec![Value::Map(vec![(
        Value::Str("name".to_string()),
        Value::Str("0xffabc".to_string()),
    )])]),
];

packet.write_to(&mut buffer).unwrap();
```

Deserialize with

```rust
let packet = vec![
    146, 162, 99, 104, 147, 165, 72, 101, 108, 108, 111, 1, 203, 63, 244, 204, 204, 204,
    204, 204, 205,
];

let val: Value = read_value_from_cursor(&mut Cursor::new(packet)).unwrap();

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
```
