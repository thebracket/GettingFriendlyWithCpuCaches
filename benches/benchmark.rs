use ardan_struct_counter::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("slow_go", |b| {
        use original_slow_go::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
    c.bench_function("fast_go", |b| {
        use original_fast_go::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
    c.bench_function("idiomatic_rust", |b| {
        use idiomatic_rust::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
    c.bench_function("no_map", |b| {
        use no_map::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
    c.bench_function("no_map_country", |b| {
        use no_map_country::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
    c.bench_function("no_map_country_idx", |b| {
        use no_map_country_idx::*;
        let users = init_users();
        b.iter(|| country_count(&users));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);