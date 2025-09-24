use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tiny_lru::TinyLru;

fn benchmark_push(c: &mut Criterion) {
    c.bench_function("push", |b| {
        b.iter(|| {
            let mut cache: TinyLru<u32, u32, 4> = TinyLru::new();
            for i in 0..4 {
                cache.push(black_box(i), black_box(i * 2));
            }
        })
    });
}

fn benchmark_get(c: &mut Criterion) {
    let mut cache: TinyLru<u32, u32, 4> = TinyLru::new();
    for i in 0..4 {
        cache.push(i, i * 2);
    }
    
    c.bench_function("get", |b| {
        b.iter(|| {
            for i in 0..4 {
                black_box(cache.get(&i));
            }
        })
    });
}

criterion_group!(benches, benchmark_push, benchmark_get);
criterion_main!(benches);
