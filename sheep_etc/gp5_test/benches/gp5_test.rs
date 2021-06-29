use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gp5_test::gp5_test;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("The Gamepro5 test", |b| {
        b.iter(|| gp5_test(black_box(&format!("hi people!")), black_box(100000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
