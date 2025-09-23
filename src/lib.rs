#![cfg_attr(not(feature = "std"), no_std)]

//! tiny-lru: A Fast Small-then-Spill LRU cache.
//!
//! This is scaffolding only. All functions are intentionally unimplemented.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::hash::Hash;

pub use tinyvec::TinyVec;

/// Intrusive node stored in the TinyVec/heap storage.
pub struct Entry<K, V> {
    pub key: K,
    pub val: V,
    pub next: u16,
    pub prev: u16,
}

/// LRU cache with inline-then-spill storage.
pub struct TinyLru<K, V, const N: usize>
where
    K: Eq + Hash,
{
    // Unified node storage; starts inline, spills to heap as capacity grows.
    // Compact: no holes; deletions swap the last element into the freed index.
    store: TinyVec<[Entry<K, V>; N]>,

    // Current number of live items.
    size: u16,

    // LRU linkage heads (indices into `store`)
    head: u16, // LRU index; sentinel if empty
    tail: u16, // MRU index; sentinel if empty

    // Key → index map. Lazily allocated ONLY on first spill to avoid heap allocs pre-spill.
    // Pre-spill lookups use linear scan over the compact TinyVec.
    index: Option<hashbrown::HashMap<K, u16>>,

    // Capacity semantics (v1 cap):
    // - size and capacity are u16; maximum capacity <= 65,534 (u16::MAX - 1)
    // - set_capacity requires new_cap > size and new_cap >= N
    capacity: u16,

    // Mode flag (informational): true once first heap allocation occurs; unspill is explicit
    is_spill: bool,
}

impl<K, V, const N: usize> TinyLru<K, V, N>
where
    K: Eq + Hash,
{
    /// Create with capacity = N.
    pub fn new() -> Self {
        todo!()
    }

    /// Create with specified capacity (must be >= N).
    pub fn with_capacity(cap: u16) -> Self {
        todo!()
    }

    /// Insert or update; promotes on hit.
    pub fn push(&mut self, key: K, value: V) {
        todo!()
    }

    /// Pop and return the LRU entry.
    pub fn pop(&mut self) -> Option<(K, V)> {
        todo!()
    }

    /// Get by key, promoting to MRU on hit.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Get mutable by key, promoting to MRU on hit.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        todo!()
    }

    /// Peek without promotion.
    pub fn peek(&self, key: &K) -> Option<&V> {
        todo!()
    }

    /// Remove by key and return owned pair.
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        todo!()
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        todo!()
    }

    /// Adjust capacity. Requires new_cap > size and new_cap >= N.
    pub fn set_capacity(&mut self, new_cap: u16) {
        todo!()
    }

    /// Current number of items.
    pub fn len(&self) -> u16 {
        todo!()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Current capacity.
    pub fn capacity(&self) -> u16 {
        todo!()
    }

    /// Contains key.
    pub fn contains_key(&self, key: &K) -> bool {
        todo!()
    }
}

// Iterators will be provided later.
// pub struct Iter<'a, K, V, const N: usize> { /* fields */ }
// pub struct IterMut<'a, K, V, const N: usize> { /* fields */ }


