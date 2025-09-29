use criterion::{criterion_group, criterion_main};

// Import benchmark modules
mod tiny_lru_bench;
mod lru_rs_bench;
mod const_lru_bench;
mod schnellru_bench;
mod uluru_bench;

// Re-export the benchmark functions
use tiny_lru_bench::{
    benchmark_push, benchmark_get, benchmark_peek, benchmark_pop
};
use lru_rs_bench::{
    benchmark_lru_rs_put, benchmark_lru_rs_get, benchmark_lru_rs_peek, benchmark_lru_rs_pop_lru
};
use const_lru_bench::{
    benchmark_const_lru_insert, benchmark_const_lru_get, benchmark_const_lru_get_untouched, benchmark_const_lru_remove
};
use schnellru_bench::{
    benchmark_schnellru_insert, benchmark_schnellru_get, benchmark_schnellru_peek, benchmark_schnellru_pop_oldest
};
use uluru_bench::{
    benchmark_uluru_insert, benchmark_uluru_find, benchmark_uluru_get
};

// Create combined benchmark groups
criterion_group!(
    tiny_lru_benches,
    benchmark_push,
    benchmark_get,
    benchmark_peek,
    benchmark_pop
);

criterion_group!(
    lru_rs_benches,
    benchmark_lru_rs_put,
    benchmark_lru_rs_get,
    benchmark_lru_rs_peek,
    benchmark_lru_rs_pop_lru
);

criterion_group!(
    const_lru_benches,
    benchmark_const_lru_insert,
    benchmark_const_lru_get,
    benchmark_const_lru_get_untouched,
    benchmark_const_lru_remove
);

criterion_group!(
    schnellru_benches,
    benchmark_schnellru_insert,
    benchmark_schnellru_get,
    benchmark_schnellru_peek,
    benchmark_schnellru_pop_oldest
);

criterion_group!(
    uluru_benches,
    benchmark_uluru_insert,
    benchmark_uluru_find,
    benchmark_uluru_get
);

// Main criterion group that includes all benchmarks
criterion_group!(
    all_benches,
    // tiny-lru benchmarks
    benchmark_push,
    benchmark_get,
    benchmark_peek,
    benchmark_pop,
    // lru-rs benchmarks
    benchmark_lru_rs_put,
    benchmark_lru_rs_get,
    benchmark_lru_rs_peek,
    benchmark_lru_rs_pop_lru,
    // const-lru benchmarks
    benchmark_const_lru_insert,
    benchmark_const_lru_get,
    benchmark_const_lru_get_untouched,
    benchmark_const_lru_remove,
    // schnellru benchmarks
    benchmark_schnellru_insert,
    benchmark_schnellru_get,
    benchmark_schnellru_peek,
    benchmark_schnellru_pop_oldest,
    // uluru benchmarks
    benchmark_uluru_insert,
    benchmark_uluru_find,
    benchmark_uluru_get
);

criterion_main!(all_benches);
