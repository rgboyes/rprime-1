use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rprime::wheel_factorize;

fn bench_wheel_factorize(c: &mut Criterion) {
    c.bench_function("factorize 1000000016000000063", |b| {
        b.iter(|| wheel_factorize(black_box(1000000016000000063u128)))
    });
}

criterion_group!(benches, bench_wheel_factorize);
criterion_main!(benches);