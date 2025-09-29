use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use uluru::LRUCache;

// Helper function to create uluru cache with N elements
fn setup_uluru_cache_with_n_elements(n: usize) -> LRUCache<u32, 128> {
    let mut cache = LRUCache::new();
    for i in 0..n {
        cache.insert((i * 2) as u32);
    }
    cache
}

// Helper function to get powers of 2 from 2 to 128
fn powers_of_2() -> Vec<usize> {
    (1..=7).map(|i| 2usize.pow(i)).collect() // 2, 4, 8, 16, 32, 64, 128
}

pub fn benchmark_uluru_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("uluru_insert");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("insert", n), &n, |b, &n| {
            b.iter_batched_ref(
                || LRUCache::<u32, 128>::new(),
                |cache| {
                    for i in 0..n {
                        cache.insert(black_box((i * 2) as u32));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_uluru_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("uluru_find");
    
    for n in powers_of_2() {
        let cache = setup_uluru_cache_with_n_elements(n);
        
        group.bench_with_input(BenchmarkId::new("find", n), &n, |b, &n| {
            b.iter_batched_ref(
                || cache.clone(),
                |cache| {
                    for i in 0..n {
                        black_box(cache.find(|&x| x == (i * 2) as u32));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

// No peek for uluru

pub fn benchmark_uluru_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("uluru_get");
    
    for n in powers_of_2() {
        let cache = setup_uluru_cache_with_n_elements(n);
        
        group.bench_with_input(BenchmarkId::new("get", n), &n, |b, &n| {
            b.iter_batched_ref(
                || cache.clone(),
                |cache| {
                    for i in 0..n {
                        black_box(cache.get(i));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(uluru_benches, benchmark_uluru_insert, benchmark_uluru_find, benchmark_uluru_get);
criterion_main!(uluru_benches);
