# Currently undergoing major changes

> [!WARNING]
> 
> MsgpackP is no longer a simple .js script, however the .js version will remain in the repository root as a relic of the time
> 
> Please. Please. PLEASE. PLEASE. DO NOT use the .js one-filer version. 
> 
> **The rust version is 900-1200 times faster**

# Building

```
cargo build
```

# Public APIs

Serialize with

```rust
let mut buffer = Vec::new();

let packet = vec![
    Value::Str("sp"),
    Value::Array(vec![Value::Map(vec![(
        Value::Str("name"),
        Value::Str("0xffabc"),
    )])]),
];

packet.write_to(&mut buffer)?;
```

Deserialize with

```rust
let packet = vec![
    146, 162, 99, 104, 147, 165, 72, 101, 108, 108, 111, 1, 203, 63, 244, 204, 204, 204,
    204, 204, 205,
];

let mut reader = Reader::new(&packet);
let val = reader.pull_value()?;

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
```
