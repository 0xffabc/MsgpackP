use criterion::{Criterion, criterion_group};
use std::hint::black_box;

use msgpackp::{msgpack::WriteTo, value::Value};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Msgpack encode");

    group.bench_function("Encode", |b| {
        let packet = vec![
            Value::Str("sp"),
            Value::Array(vec![Value::Array(vec![
                Value::Str("name"),
                Value::Str("0xffabc"),
            ])]),
        ];

        b.iter(move || {
            let mut buffer = Vec::new();

            black_box(packet.write_to(&mut buffer).unwrap());
        });
    });

    group.finish();
}

criterion_group!(benches, bench);
