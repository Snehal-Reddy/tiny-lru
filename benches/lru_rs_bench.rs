use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lru::LruCache;
use std::num::NonZeroUsize;

// Helper function to create lru-rs cache with N elements
fn setup_lru_rs_cache_with_n_elements(n: usize) -> LruCache<u32, u32> {
    let mut cache = LruCache::new(NonZeroUsize::new(128).unwrap());
    for i in 0..n {
        cache.put(i as u32, (i * 2) as u32);
    }
    cache
}

// Helper function to get powers of 2 from 2 to 128
fn powers_of_2() -> Vec<usize> {
    (1..=7).map(|i| 2usize.pow(i)).collect() // 2, 4, 8, 16, 32, 64, 128
}

pub fn benchmark_lru_rs_put(c: &mut Criterion) {
    let mut group = c.benchmark_group("lru_rs_put");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("put", n), &n, |b, &n| {
            b.iter_batched_ref(
                || LruCache::new(NonZeroUsize::new(128).unwrap()),
                |cache| {
                    for i in 0..n {
                        cache.put(black_box(i as u32), black_box((i * 2) as u32));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_lru_rs_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("lru_rs_get");
    
    for n in powers_of_2() {
        let cache = setup_lru_rs_cache_with_n_elements(n);
        
        group.bench_with_input(BenchmarkId::new("get", n), &n, |b, &n| {
            b.iter_batched_ref(
                || cache.clone(),
                |cache| {
                    for i in 0..n {
                        black_box(cache.get(&(i as u32)));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_lru_rs_peek(c: &mut Criterion) {
    let mut group = c.benchmark_group("lru_rs_peek");
    
    for n in powers_of_2() {
        let cache = setup_lru_rs_cache_with_n_elements(n);
        
        group.bench_with_input(BenchmarkId::new("peek", n), &n, |b, &n| {
            b.iter_batched_ref(
                || cache.clone(),
                |cache| {
                    for i in 0..n {
                        black_box(cache.peek(&(i as u32)));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_lru_rs_pop_lru(c: &mut Criterion) {
    let mut group = c.benchmark_group("lru_rs_pop_lru");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("pop_lru", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_lru_rs_cache_with_n_elements(n),
                |cache| {
                    for _ in 0..n {
                        black_box(cache.pop_lru());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(lru_rs_benches, benchmark_lru_rs_put, benchmark_lru_rs_get, benchmark_lru_rs_peek, benchmark_lru_rs_pop_lru);
criterion_main!(lru_rs_benches);
