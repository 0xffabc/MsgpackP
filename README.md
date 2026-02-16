# MSgpackP

An **unsafe** and fast msgpack implementation

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

    let mut buffer = [0u8; 70];
    let mut slice = &mut buffer[..];

    packet.write_to(&mut slice).unwrap();
```

Deserialize with

```rust
let packet = &[
  135, 163, 105, 110, 116, 1, 165, 102, 108, 111, 97, 116, 203, 63, 224, 0, 0, 0, 0,
  0, 0, 167, 98, 111, 111, 108, 101, 97, 110, 195, 164, 110, 117, 108, 108, 192, 166,
  115, 116, 114, 105, 110, 103, 167, 102, 111, 111, 32, 98, 97, 114, 165, 97, 114,
  114, 97, 121, 146, 163, 102, 111, 111, 163, 98, 97, 114, 166, 111, 98, 106, 101,
  99, 116, 130, 163, 102, 111, 111, 1, 163, 98, 97, 122, 203, 63, 224, 0, 0, 0, 0, 0,
  0,
];

let mut reader = Reader::new(packet);

let value = reader.pull_value();

println!("{value}");
```
