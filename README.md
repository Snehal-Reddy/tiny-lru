# tiny-lru

A Fast Small-then-Spill LRU cache combining the raw speed of stack-based LRUs with the scalability of heap-backed ones.

## Design Philosophy

For very small working sets, entries are **stored inline on the stack** in a fixed-capacity array, giving fast, allocation-free lookups and updates with excellent cache locality. Once the inline capacity is exceeded, it transparently "spills" into a **heap-backed LRU (hash map + linked list)**, ensuring O(1) operations at larger scales.

The design goal is zero compromise on micro-performance for small caches while still supporting larger workloads without falling off a performance cliff. In short: a tinyvec-style hybrid LRU optimized for both tiny hot paths (embedded, HFT, real-time) and unbounded dynamic growth when needed.

## Pre-Spill Performance (Small Cache Sizes)

Performance comparison showing relative speed (higher numbers = slower). tiny-lru is the baseline (1.00).

### Push Operations
| Implementation | 2 | 4 | 8 | 16 | 32 |
|------------|------|------|------|------|------|
| tiny-lru 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| const-lru | 2.00 | 2.39 | 2.68 | 2.90 | 2.65 |
| lru-rs | 6.96 | 7.22 | 6.02 | 5.49 | 3.67 |
| schnellru | 2.36 | 6.42 | 7.84 | 7.66 | 5.61 |

### Pop Operations
| Implementation | 2 | 4 | 8 | 16 | 32 |
|------------|------|------|------|------|------|
| tiny-lru 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| const-lru | 2.58 | 3.76 | 3.53 | 3.84 | 5.17 |
| lru-rs | 14.05 | 9.33 | 7.10 | 4.13 | 3.58 |
| schnellru | 1.15 | 1.31 | 1.33 | 1.11 | 0.83 |

### Peek Operations
| Implementation | 2 | 4 | 8 | 16 | 32 |
|------------|------|------|------|------|------|
| tiny-lru 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| const-lru | 1.90 | 2.37 | 2.03 | 1.56 | 1.16 |
| lru-rs | 3.58 | 4.29 | 3.47 | 2.25 | 1.12 |
| schnellru | 1.16 | 1.60 | 1.60 | 1.20 | 0.77 |

### Get Operations
| Implementation | 2 | 4 | 8 | 16 | 32 |
|------------|------|------|------|------|------|
| tiny-lru 👍 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| const-lru | 2.00 | 3.02 | 2.80 | 1.77 | 1.30 |
| lru-rs | 3.11 | 3.08 | 2.56 | 1.46 | 0.77 |
| schnellru | 0.98 | 1.14 | 1.21 | 0.90 | 0.60 |

*More benchmarks coming soon - will compare against standard `lru` crate and other implementations*

### Benchmark Details

- **Hardware**: Core Ultra 7 265KF, 32 GB DDR5-6000
- **Compiler/tooling**: Rust edition `2024`; Criterion `0.5` (compiler version not pinned in repo)
- **How to run**: `cargo bench` (benches declared in `Cargo.toml` under `[[bench]]`)

## Use Cases

- **Embedded systems**: Configurable memory footprint
- **Real-time systems**: Predictable performance characteristics
- **General caching**: Drop-in replacement for standard LRU with better small-cache performance
