use criterion::{Criterion, criterion_group};
use std::{hint::black_box, io::Cursor};

use msgpackp::reader;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Msgpack decode");

    group.bench_function("Decode", |b| {
        let packet = vec![
            146, 162, 99, 104, 147, 165, 72, 101, 108, 108, 111, 1, 203, 63, 244, 204, 204, 204,
            204, 204, 205,
        ];

        b.iter(move || {
            let mut cursor = Cursor::new(black_box(packet.clone()));

            black_box(reader::read_value_from_cursor(&mut cursor)).unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, bench);
