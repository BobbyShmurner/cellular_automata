use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod main;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Mesh Gen 20", |b| b.iter(|| main::generate_mesh(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);-