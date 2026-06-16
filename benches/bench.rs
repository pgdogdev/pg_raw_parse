use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn benchmark(c: &mut Criterion) {
    let ids = (0..1_300_000)
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!("SELECT COUNT(*) FROM users WHERE id IN ({ids})");
    let mut group = c.benchmark_group("parsers");
    group.bench_function("pg_raw_parse::parse", |b| {
        b.iter(|| pg_raw_parse::parse(black_box(&*sql)).unwrap())
    });
    group.bench_function("pg_query::parse", |b| {
        b.iter(|| pg_query::parse(black_box(&*sql)).unwrap())
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
