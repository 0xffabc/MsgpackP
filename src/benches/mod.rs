use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::msgpackp_encode::benches,
    benchmarks::msgpackp_decode::benches
}
