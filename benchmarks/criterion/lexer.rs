use criterion::{criterion_group, criterion_main, Criterion};

fn bench_lexer(c: &mut Criterion) {
    c.bench_function("lexer", |b| b.iter(|| ravensone::lexer::init()));
}

criterion_group!(benches, bench_lexer);
criterion_main!(benches);
