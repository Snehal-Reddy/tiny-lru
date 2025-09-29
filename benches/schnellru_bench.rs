use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use schnellru::{LruMap, ByLength};

// Helper function to create schnellru cache with N elements
fn setup_schnellru_cache_with_n_elements(n: usize) -> LruMap<u32, u32, ByLength> {
    let mut cache = LruMap::new(ByLength::new(128));
    for i in 0..n {
        cache.insert(i as u32, (i * 2) as u32);
    }
    cache
}

// Helper function to get powers of 2 from 2 to 128
fn powers_of_2() -> Vec<usize> {
    (1..=7).map(|i| 2usize.pow(i)).collect() // 2, 4, 8, 16, 32, 64, 128
}

pub fn benchmark_schnellru_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("schnellru_insert");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("insert", n), &n, |b, &n| {
            b.iter_batched_ref(
                || LruMap::new(ByLength::new(128)),
                |cache| {
                    for i in 0..n {
                        cache.insert(black_box(i as u32), black_box((i * 2) as u32));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_schnellru_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("schnellru_get");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("get", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_schnellru_cache_with_n_elements(n),
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

pub fn benchmark_schnellru_peek(c: &mut Criterion) {
    let mut group = c.benchmark_group("schnellru_peek");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("peek", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_schnellru_cache_with_n_elements(n),
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

pub fn benchmark_schnellru_pop_oldest(c: &mut Criterion) {
    let mut group = c.benchmark_group("schnellru_pop_oldest");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("pop_oldest", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_schnellru_cache_with_n_elements(n),
                |cache| {
                    for _ in 0..n {
                        black_box(cache.pop_oldest());
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(schnellru_benches, benchmark_schnellru_insert, benchmark_schnellru_get, benchmark_schnellru_peek, benchmark_schnellru_pop_oldest);
criterion_main!(schnellru_benches);
