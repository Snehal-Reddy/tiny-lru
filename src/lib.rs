#![cfg_attr(not(feature = "std"), no_std)]
#![feature(portable_simd)]

//! tiny-lru: A Fast Small-then-Spill LRU cache.

extern crate alloc;

use core::hash::Hash;
use tinyvec::TinyVec;

/// Helper function to get hash for any Hash type using foldhash
/// TODO: Maybe use passthrough hash for simpler types?
pub fn get_key_hash<K: Hash>(key: &K) -> u64 {
    use foldhash::fast::FixedState;
    use core::hash::{BuildHasher, Hasher};
    let mut hasher = FixedState::with_seed(0).build_hasher();
    key.hash(&mut hasher);
    hasher.finish()
}

/// Maximum capacity for v1 implementation (u16::MAX - 1)
const MAX_CAPACITY: u16 = u16::MAX - 1;

/// LRU cache with inline-then-spill storage using Struct of Arrays (SoA) layout.
/// 
/// SoA layout enables SIMD-optimized hash-based lookups and better cache utilization
/// during key searches by storing related data in separate arrays.
#[derive(Clone)]
pub struct TinyLru<K, V, const N: usize>
where
    K: Eq + Hash + Default,
    V: Default,
{
    // Separate arrays for each component - enables SIMD optimization
    // CRITICAL: All arrays remain COMPACT with no holes, using swap-remove for deletions
    hashes: TinyVec<[u64; N]>,        // Pre-computed hashes for SIMD comparison
    keys: TinyVec<[K; N]>,            // Actual keys (accessed only on hash collision)
    values: TinyVec<[V; N]>,          // Values
    next: TinyVec<[u16; N]>,          // DLL next pointers
    prev: TinyVec<[u16; N]>,          // DLL prev pointers
    
    // Metadata (unchanged from AoS)
    size: u16,                        // Current number of live items
    head: u16,                        // LRU index; sentinel if empty
    tail: u16,                        // MRU index; sentinel if empty
    capacity: u16,                    // Total capacity (≤ 65,534)
    is_spill: bool,                   // Informational spill flag
    index: Option<hashbrown::HashMap<K, u16>>, // Lazily allocated on first spill
}

// Compile-time assertion: N must be <= MAX_CAPACITY
const fn assert_capacity_limit<const N: usize>() {
    assert!(N <= MAX_CAPACITY as usize, "N must be <= MAX_CAPACITY for v1 capacity limits");
}

impl<K, V, const N: usize> TinyLru<K, V, N>
where
    K: Eq + Hash + Default,
    V: Default,
{
    /// Create with capacity = N.
    pub fn new() -> Self {
        assert_capacity_limit::<N>();
        
        Self {
            hashes: TinyVec::new(),
            keys: TinyVec::new(),
            values: TinyVec::new(),
            next: TinyVec::new(),
            prev: TinyVec::new(),
            size: 0,
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            capacity: N as u16,
            is_spill: false,
            index: None,    // No HashMap allocated pre-spill
        }
    }

    /// Create with specified capacity (must be >= N).
    pub fn with_capacity(cap: u16) -> Self {
        assert_capacity_limit::<N>();
        
        // Runtime assertion: capacity must be >= N
        assert!(cap >= N as u16, "capacity must be >= N");
        
        Self {
            hashes: TinyVec::new(),
            keys: TinyVec::new(),
            values: TinyVec::new(),
            next: TinyVec::new(),
            prev: TinyVec::new(),
            size: 0,
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            capacity: cap,
            is_spill: false,
            index: None,    // No HashMap allocated pre-spill
        }
    }

    /// Insert or update; promotes on hit.
    pub fn push(&mut self, key: K, value: V) {
        // If key exists: update value and promote to MRU
        if let Some(index) = self.find_key_index(&key) {
            // Update the value and hash
            self.values[index] = value;
            self.hashes[index] = get_key_hash(&key);
            // Promote to MRU (move to tail)
            self.promote_to_mru(index);
            return;
        }

        // Key doesn't exist - need to insert new entry
        if self.size < self.capacity {
            // Cache has space - insert directly
            if self.size < N as u16 {
                // Pre-spill: store inline (no allocations)
                self.insert_inline(key, value);
            } else {
                // Post-spill: use heap storage
                todo!("spill: insert new entry in heap mode");
            }
        } else {
            // Cache is full - evict LRU first, then insert
            if self.is_spill {
                todo!("spill: evict LRU entry and insert new entry in heap mode");
            } else {
                // Pre-spill: evict LRU, then insert new entry
                self.pop(); // Remove LRU entry
                self.insert_inline(key, value); // Insert new entry
            }
        }
    }

    /// Pop and return the LRU entry.
    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.is_spill {
            todo!("pop: post-spill pop not implemented yet")
        }

        if self.is_empty() {
            return None;
        }

        // Get the LRU index (head)
        let lru_index = self.head as usize;
        // Capture next before swap_remove
        let next_index_before = self.next[lru_index];
        let last_index_before = (self.size - 1) as usize;
        
        // Extract the key-value pair before removal
        let key = self.keys.swap_remove(lru_index);
        let value = self.values.swap_remove(lru_index);
        let _hash = self.hashes.swap_remove(lru_index);
        let _next = self.next.swap_remove(lru_index);
        let _prev = self.prev.swap_remove(lru_index);
        
        // Update size
        self.size -= 1;
        
        // Handle DLL updates
        if self.size == 0 {
            // Last element removed - reset to empty state
            self.head = u16::MAX;
            self.tail = u16::MAX;
        } else {
            // Update head to next element (adjust if it pointed at the old last index)
            if lru_index < last_index_before && next_index_before as usize == last_index_before {
                // The old next was the last element which moved into lru_index
                self.head = lru_index as u16;
            } else {
                self.head = next_index_before;
            }
            if self.head != u16::MAX {
                self.prev[self.head as usize] = u16::MAX;
            }
            
            // If we swapped with the last element, update its index in the DLL
            if lru_index < self.size as usize {
                self.update_swapped_element_index(lru_index);
            }
        }
        
        Some((key, value))
    }

    /// Get by key, promoting to MRU on hit.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.is_spill {
            todo!("post-spill get: use hashmap index and promote to MRU")
        }

        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&self.values[index])
        } else {
            None
        }
    }

    /// Get mutable by key, promoting to MRU on hit.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.is_spill {
            todo!("post-spill get_mut: use hashmap index and promote to MRU")
        }

        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    /// Peek without promotion.
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.find_key_index(key).map(|index| &self.values[index])
    }

    /// Remove by key and return owned pair.
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        if self.is_spill {
            todo!("remove: post-spill removal not implemented yet")
        }

        // Find the key index
        let index = self.find_key_index(key)?;
        
        let last_index_before = (self.size - 1) as usize;
        let removed_prev = self.prev[index];
        let removed_next = self.next[index];
        // Extract the key-value pair before removal
        let key = self.keys.swap_remove(index);
        let value = self.values.swap_remove(index);
        let _hash = self.hashes.swap_remove(index);
        let _next = self.next.swap_remove(index);
        let _prev = self.prev.swap_remove(index);
        
        // Update size
        self.size -= 1;
        
        // Handle DLL updates
        if self.size == 0 {
            // Last element removed - reset to empty state
            self.head = u16::MAX;
            self.tail = u16::MAX;
        } else {
            // Update DLL pointers. Adjust for swap if needed.
            let (mut prev, mut next) = (removed_prev, removed_next);
            if index < last_index_before {
                // The last element moved into `index`. If removed_prev/next referred to the old last index,
                // redirect them to `index` now.
                if prev as usize == last_index_before { prev = index as u16; }
                if next as usize == last_index_before { next = index as u16; }
            }
            self.remove_from_dll(index, prev, next);

            // If we swapped with the last element, update its index in the DLL
            if index < self.size as usize {
                self.update_swapped_element_index(index);
            }
        }
        
        Some((key, value))
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        // Clear all arrays efficiently
        self.hashes.clear();
        self.keys.clear();
        self.values.clear();
        self.next.clear();
        self.prev.clear();
        
        // Reset state to empty
        self.size = 0;
        self.head = u16::MAX; // Sentinel value for empty list
        self.tail = u16::MAX; // Sentinel value for empty list
        
        // Clear HashMap index for simplicity
        self.index = None;
        self.is_spill = false;
    }

    /// Adjust capacity. Requires new_cap > size and new_cap >= N.
    pub fn set_capacity(&mut self, new_cap: u16) {
        // Validate requirements
        assert!(new_cap > self.size, "new_cap must be > current size");
        assert!(new_cap >= N as u16, "new_cap must be >= N");
        
        if self.is_spill {
            todo!("set_capacity: post-spill capacity adjustment not implemented yet")
        }
        
        // Pre-spill: just update the capacity field
        // The TinyVec will handle spill automatically when we exceed N
        self.capacity = new_cap;
    }

    /// Current number of items.
    pub fn len(&self) -> u16 {
        self.size
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Current capacity.
    pub fn capacity(&self) -> u16 {
        self.capacity
    }

    /// Contains key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.find_key_index(key).is_some()
    }

    /// Whether the cache has spilled to heap storage.
    pub fn is_spill(&self) -> bool {
        self.is_spill
    }


    /// Find the index of a key.
    /// - Pre-spill: SIMD-optimized hash-based scan over compact TinyVec
    /// - Post-spill: todo!() to use hashmap index for O(1)
    /// Returns None if key not found.
    fn find_key_index(&self, key: &K) -> Option<usize> {
        if self.is_spill {
            // Post-spill: look up via hashmap index
            todo!("post-spill find_key_index: use hashmap index for O(1) lookup")
        } else {
            // Pre-spill: SIMD-optimized hash-based scan over compact TinyVec
            let target_hash = get_key_hash(key);
            self.find_key_index_simd(key, target_hash)
        }
    }

    /// SIMD-optimized hash comparison for find_key_index
    fn find_key_index_simd(&self, key: &K, target_hash: u64) -> Option<usize> {
        use std::simd::{Simd, Mask, cmp::SimdPartialEq};
        
        // Use u64x4 for 4-way parallel hash comparison
        const LANES: usize = 4;
        let len = self.hashes.len();
        
        if len == 0 {
            return None;
        }
        
        // For very small arrays, use scalar comparison to avoid SIMD overhead
        if len < LANES {
            for i in 0..len {
                if self.hashes[i] == target_hash && self.keys[i] == *key {
                    return Some(i);
                }
            }
            return None;
        }
        
        // Create SIMD vector with target hash repeated
        let target_simd = Simd::splat(target_hash);
        
        // Process chunks of 4 hashes at a time
        let mut i = 0;
        while i + LANES <= len {
            // Load chunk of hashes into SIMD vector
            let chunk = &self.hashes[i..i + LANES];
            let hashes_simd = Simd::from_slice(chunk);
            
            // Compare hashes in parallel
            let mask: Mask<i64, LANES> = hashes_simd.simd_eq(target_simd);
            
            // Check if any hashes match
            if mask.any() {
                // Check each lane for hash matches
                for lane in 0..LANES {
                    if mask.test(lane) {
                        let idx = i + lane;
                        // Hash match - verify with actual key comparison
                        if self.keys[idx] == *key {
                            return Some(idx);
                        }
                    }
                }
            }
            
            i += LANES;
        }
        
        // Handle remaining elements with scalar comparison
        for j in i..len {
            if self.hashes[j] == target_hash && self.keys[j] == *key {
                return Some(j);
            }
        }
        
        None
    }

    /// Insert a new entry inline (pre-spill only).
    fn insert_inline(&mut self, key: K, value: V) {
        let new_index = self.size as usize;
        
        // Pre-compute hash for fast lookups
        let hash = get_key_hash(&key);

        // Add to all arrays
        self.hashes.push(hash);
        self.keys.push(key);
        self.values.push(value);
        self.next.push(u16::MAX); // Will be set to current tail
        self.prev.push(self.tail); // Previous MRU

        // Update linked list
        if self.size == 0 {
            // First entry - set as both head and tail
            self.head = new_index as u16;
            self.tail = new_index as u16;
        } else {
            // Link to previous tail
            self.next[self.tail as usize] = new_index as u16;
            self.tail = new_index as u16;
        }

        // Update size
        self.size += 1;
    }

    /// Promote an entry to MRU (move to tail).
    fn promote_to_mru(&mut self, index: usize) {
        // Early return if already MRU or only one element
        if self.size <= 1 || index == self.tail as usize {
            return;
        }

        let entry_index = index as u16;
        // Copy links first to avoid holding immutable borrows during mutation
        let prev = self.prev[index];
        let next = self.next[index];

        // Remove from current position
        if entry_index == self.head {
            // Moving head - update head to next
            self.head = next;
            if self.head != u16::MAX {
                self.prev[self.head as usize] = u16::MAX;
            }
        } else {
            // Update previous entry's next pointer
            self.next[prev as usize] = next;
        }

        // Update next entry's prev pointer
        if next != u16::MAX {
            self.prev[next as usize] = prev;
        }

        // Move to tail
        self.prev[index] = self.tail;
        self.next[index] = u16::MAX;
        
        // Update previous tail's next pointer
        self.next[self.tail as usize] = entry_index;
        
        // Update tail
        self.tail = entry_index;
    }

    /// Remove a node from the doubly-linked list.
    fn remove_from_dll(&mut self, _index: usize, prev: u16, next: u16) {
        // Update previous node's next pointer
        if prev != u16::MAX {
            self.next[prev as usize] = next;
        } else {
            // This was the head - update head
            self.head = next;
        }
        
        // Update next node's prev pointer
        if next != u16::MAX {
            self.prev[next as usize] = prev;
        } else {
            // This was the tail - update tail
            self.tail = prev;
        }
    }

    /// Update the index of a swapped element in the DLL.
    fn update_swapped_element_index(&mut self, old_index: usize) {
        let new_index = old_index;
        
        // Copy the prev/next values to avoid borrow conflicts
        let prev = self.prev[new_index];
        let next = self.next[new_index];
        
        // Update references to this element
        if prev != u16::MAX {
            self.next[prev as usize] = new_index as u16;
        } else {
            // This is now the head
            self.head = new_index as u16;
        }
        
        if next != u16::MAX {
            self.prev[next as usize] = new_index as u16;
        } else {
            // This is now the tail
            self.tail = new_index as u16;
        }
    }
}

// Iterators will be provided later.
// pub struct Iter<'a, K, V, const N: usize> { /* fields */ }
// pub struct IterMut<'a, K, V, const N: usize> { /* fields */ }

#[cfg(test)]
mod tests;


