use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tiny_lru::TinyLru;

// Helper function to create cache with N elements
fn setup_cache_with_n_elements<const N: usize>(n: usize) -> TinyLru<u32, u32, N> {
    let mut cache = TinyLru::new();
    for i in 0..n {
        cache.push(i as u32, (i * 2) as u32);
    }
    cache
}

// Helper function to get powers of 2 from 2 to 128
fn powers_of_2() -> Vec<usize> {
    (1..=7).map(|i| 2usize.pow(i)).collect() // 2, 4, 8, 16, 32, 64, 128
}

pub fn benchmark_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("push");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("push", n), &n, |b, &n| {
            b.iter_batched_ref(
                || TinyLru::<u32, u32, 128>::new(),
                |cache| {
                    for i in 0..n {
                        cache.push(black_box(i as u32), black_box((i * 2) as u32));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");
    
    for n in powers_of_2() {
        let cache = setup_cache_with_n_elements::<128>(n);
        
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

pub fn benchmark_peek(c: &mut Criterion) {
    let mut group = c.benchmark_group("peek");
    
    for n in powers_of_2() {
        let cache = setup_cache_with_n_elements::<128>(n);
        
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

pub fn benchmark_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("pop", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_cache_with_n_elements::<128>(n),
                |cache| {
                    for _ in 0..n {
                        black_box(cache.pop());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(tiny_lru_benches, benchmark_push, benchmark_get, benchmark_peek, benchmark_pop);
criterion_main!(tiny_lru_benches);
