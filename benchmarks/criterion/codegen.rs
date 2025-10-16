use criterion::{criterion_group, criterion_main, Criterion};

fn bench_codegen(c: &mut Criterion) {
    c.bench_function("codegen", |b| b.iter(|| ravensone::codegen::init()));
}

criterion_group!(benches, bench_codegen);
criterion_main!(benches);
