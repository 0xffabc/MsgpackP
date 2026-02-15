use criterion::{Criterion, criterion_group};
use std::hint::black_box;

use msgpackp::{msgpack::WriteTo, value::Value};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Msgpack encode");

    group.bench_function("Encode", |b| {
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

        b.iter(move || {
            let mut buffer = [0u8; 70];
            let mut slice = &mut buffer[..];

            black_box(packet.write_to(&mut slice).unwrap());
        });
    });

    group.finish();
}

criterion_group!(benches, bench);
