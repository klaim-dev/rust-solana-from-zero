use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_vec_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_capacity");
    let n = 100_000usize;

    group.bench_function(BenchmarkId::new("no_cap", n), |b| {
        b.iter(|| {
            let v = day29_tooling::build_vec_no_cap(black_box(n));
            black_box(v);
        });
    });

    group.bench_function(BenchmarkId::new("with_cap", n), |b| {
        b.iter(|| {
            let v = day29_tooling::build_vec_with_cap(black_box(n));
            black_box(v);
        });
    });

    group.finish();
}

#[cfg(feature = "bench-string")]
fn bench_string_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_capacity");
    let repeats = 1_000usize;
    let chunk = "0123456789abcdef";

    group.bench_function(BenchmarkId::new("no_cap", repeats), |b| {
        b.iter(|| {
            let s = day29_tooling::build_string_no_cap(black_box(repeats), black_box(chunk));
            black_box(s);
        });
    });

    group.bench_function(BenchmarkId::new("with_cap", repeats), |b| {
        b.iter(|| {
            let s = day29_tooling::build_string_with_cap(black_box(repeats), black_box(chunk));
            black_box(s);
        });
    });

    group.finish();
}

#[cfg(feature = "bench-string")]
criterion_group!(benches, bench_vec_capacity, bench_string_capacity);

#[cfg(not(feature = "bench-string"))]
criterion_group!(benches, bench_vec_capacity);

criterion_main!(benches);
