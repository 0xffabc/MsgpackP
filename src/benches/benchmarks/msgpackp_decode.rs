use criterion::{Criterion, criterion_group};
use std::{hint::black_box, io::Cursor};

use msgpackp::reader;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Msgpack decode");

    group.bench_function("Decode", |b| {
        b.iter(move || {
            let packet = &[
                146u8, 162, 99, 104, 147, 165, 72, 101, 108, 108, 111, 1, 203, 63, 244, 204, 204,
                204, 204, 204, 205,
            ];

            let mut cursor = Cursor::new(packet);

            black_box(reader::read_value_from_cursor(&mut cursor)).unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, bench);
