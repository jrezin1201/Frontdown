use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parser(c: &mut Criterion) {
    c.bench_function("parser", |b| b.iter(|| ravensone::parser::init()));
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
