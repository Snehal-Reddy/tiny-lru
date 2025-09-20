# tiny-lru

A Fast Small-then-Spill LRU cache combining the raw speed of stack-based LRUs with the scalability of heap-backed ones.

## Design Philosophy

For very small working sets, entries are stored inline in a fixed-capacity array, giving blazing-fast, allocation-free lookups and updates with excellent cache locality. Once the inline capacity is exceeded, it transparently "spills" into a heap-backed LRU (hash map + linked list), ensuring O(1) operations at larger scales.

The design goal is zero compromise on micro-performance for small caches while still supporting larger workloads without falling off a performance cliff. In short: a tinyvec-style hybrid LRU optimized for both tiny hot paths (HFT, embedded, real-time) and unbounded dynamic growth when needed.

*Benchmarks coming soon - will compare against standard `lru` crate and other implementations*

## Use Cases

- **Embedded systems**: Configurable memory footprint
- **Real-time systems**: Predictable performance characteristics
- **High-frequency trading**: Ultra-low latency for small hot caches
- **General caching**: Drop-in replacement for standard LRU with better small-cache performance
