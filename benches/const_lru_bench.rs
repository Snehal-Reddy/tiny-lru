use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use const_lru::ConstLru;

// Helper function to create const-lru cache with N elements
fn setup_const_lru_cache_with_n_elements(n: usize) -> ConstLru<u32, u32, 128> {
    let mut cache = ConstLru::new();
    for i in 0..n {
        cache.insert(i as u32, (i * 2) as u32);
    }
    cache
}

// Helper function to get powers of 2 from 2 to 128
fn powers_of_2() -> Vec<usize> {
    (1..=7).map(|i| 2usize.pow(i)).collect() // 2, 4, 8, 16, 32, 64, 128
}

pub fn benchmark_const_lru_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("const_lru_insert");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("insert", n), &n, |b, &n| {
            b.iter_batched_ref(
                || ConstLru::<u32, u32, 128>::new(),
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

pub fn benchmark_const_lru_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("const_lru_get");
    
    for n in powers_of_2() {
        let cache = setup_const_lru_cache_with_n_elements(n);
        
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

pub fn benchmark_const_lru_get_untouched(c: &mut Criterion) {
    let mut group = c.benchmark_group("const_lru_get_untouched");
    
    for n in powers_of_2() {
        let cache = setup_const_lru_cache_with_n_elements(n);
        
        group.bench_with_input(BenchmarkId::new("get_untouched", n), &n, |b, &n| {
            b.iter_batched_ref(
                || cache.clone(),
                |cache| {
                    for i in 0..n {
                        black_box(cache.get_untouched(&(i as u32)));
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

pub fn benchmark_const_lru_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("const_lru_remove");
    
    for n in powers_of_2() {
        group.bench_with_input(BenchmarkId::new("remove", n), &n, |b, &n| {
            b.iter_batched_ref(
                || setup_const_lru_cache_with_n_elements(n),
                |cache| {
                    for _ in 0..n {
                        // Replicate pop_lru() by getting LRU key first, then removing it
                        if let Some((lru_key, _)) = cache.iter().last() {
                            let key_to_remove = *lru_key; // Clone the key to avoid borrow conflict
                            black_box(cache.remove(&key_to_remove));
                        }
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    group.finish();
}

criterion_group!(const_lru_benches, benchmark_const_lru_insert, benchmark_const_lru_get, benchmark_const_lru_get_untouched, benchmark_const_lru_remove);
criterion_main!(const_lru_benches);
