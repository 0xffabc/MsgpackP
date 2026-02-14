use criterion::{Criterion, criterion_group};
use std::{hint::black_box, io::Cursor};

use msgpackp::reader;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Msgpack decode");

    group.bench_function("Decode", |b| {
        b.iter(move || {
            let packet = &[
                135, 163, 105, 110, 116, 1, 165, 102, 108, 111, 97, 116, 203, 63, 224, 0, 0, 0, 0,
                0, 0, 167, 98, 111, 111, 108, 101, 97, 110, 195, 164, 110, 117, 108, 108, 192, 166,
                115, 116, 114, 105, 110, 103, 167, 102, 111, 111, 32, 98, 97, 114, 165, 97, 114,
                114, 97, 121, 146, 163, 102, 111, 111, 163, 98, 97, 114, 166, 111, 98, 106, 101,
                99, 116, 130, 163, 102, 111, 111, 1, 163, 98, 97, 122, 203, 63, 224, 0, 0, 0, 0, 0,
                0,
            ];

            let mut cursor = Cursor::new(packet);

            black_box(reader::read_value_from_cursor(&mut cursor)).unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, bench);
