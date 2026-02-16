use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::msgpackp_decode::benches,
    benchmarks::msgpackp_encode::benches,
}
