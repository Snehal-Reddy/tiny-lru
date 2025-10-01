#![cfg_attr(not(feature = "std"), no_std)]

//! tiny-lru: A Fast Small-then-Spill LRU cache.
//!
//! This is scaffolding only. All functions are intentionally unimplemented.

extern crate alloc;

use core::hash::Hash;
use tinyvec::TinyVec;

/// Maximum capacity for v1 implementation (u16::MAX - 1)
const MAX_CAPACITY: u16 = u16::MAX - 1;

/// Intrusive node stored in the TinyVec/heap storage.
#[derive(Default, Clone)]
pub struct Entry<K, V> 
where
    K: Default,
    V: Default,
{
    pub key: K,
    pub val: V,
    pub next: u16,
    pub prev: u16,
}

/// LRU cache with inline-then-spill storage.
#[derive(Clone)]
pub struct TinyLru<K, V, const N: usize>
where
    K: Eq + Hash + Default,
    V: Default,
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
            store: TinyVec::new(),
            size: 0,
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            index: None,    // No HashMap allocated pre-spill
            capacity: N as u16,
            is_spill: false,
        }
    }

    /// Create with specified capacity (must be >= N).
    pub fn with_capacity(cap: u16) -> Self {
        assert_capacity_limit::<N>();
        
        // Runtime assertion: capacity must be >= N
        assert!(cap >= N as u16, "capacity must be >= N");
        
        Self {
            store: TinyVec::new(),
            size: 0,
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            index: None,    // No HashMap allocated pre-spill
            capacity: cap,
            is_spill: false,
        }
    }

    /// Insert or update; promotes on hit.
    pub fn push(&mut self, key: K, value: V) {
        // If key exists: update value and promote to MRU
        if let Some(index) = self.find_key_index(&key) {
            // Update the value
            self.store[index].val = value;
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
        let next_index_before = self.store[lru_index].next;
        let last_index_before = (self.size - 1) as usize;
        
        // Extract the key-value pair before removal
        let entry = self.store.swap_remove(lru_index);
        let (key, value) = (entry.key, entry.val);
        
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
                self.store[self.head as usize].prev = u16::MAX;
            }
            
            // If we swapped with the last element, update its index in the DLL
            if lru_index < self.size as usize {
                self.update_swapped_element_index(lru_index);
            }
        }
        
        Some((key, value))
    }

    /// Get by key, promoting to MRU on hit.
    #[inline(always)]
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.is_spill {
            todo!("post-spill get: use hashmap index and promote to MRU")
        }

        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&self.store[index].val)
        } else {
            None
        }
    }

    /// Get mutable by key, promoting to MRU on hit.
    #[inline(always)]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.is_spill {
            todo!("post-spill get_mut: use hashmap index and promote to MRU")
        }

        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&mut self.store[index].val)
        } else {
            None
        }
    }

    /// Peek without promotion.
    #[inline(always)]
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.find_key_index(key).map(|index| &self.store[index].val)
    }

    /// Remove by key and return owned pair.
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        if self.is_spill {
            todo!("remove: post-spill removal not implemented yet")
        }

        // Find the key index
        let index = self.find_key_index(key)?;
        
        let last_index_before = (self.size - 1) as usize;
        let removed_prev = self.store[index].prev;
        let removed_next = self.store[index].next;
        // Extract the key-value pair before removal
        let entry = self.store.swap_remove(index);
        let (key, value) = (entry.key, entry.val);
        
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
        // Clear the store efficiently
        self.store.clear();
        
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
    #[inline(always)]
    pub fn contains_key(&self, key: &K) -> bool {
        self.find_key_index(key).is_some()
    }


    /// Find the index of a key.
    /// - Pre-spill: linear scan over compact TinyVec
    /// - Post-spill: todo!() to use hashmap index for O(1)
    /// Returns None if key not found.
    #[inline(always)]
    fn find_key_index(&self, key: &K) -> Option<usize> {
        if self.is_spill {
            // Post-spill: look up via hashmap index
            todo!("post-spill find_key_index: use hashmap index for O(1) lookup")
        } else {
            // Pre-spill: use raw slice iteration
            let entries = &self.store[..self.size as usize];
            for i in 0..entries.len() {
                // SAFETY: We iterate over `0..entries.len()` where `entries` is a slice of length
                // `self.size as usize`. Since `i` is guaranteed to be in bounds of this slice,
                // `get_unchecked(i)` is safe. This avoids bounds checks in the hot path.
                let entry = unsafe { entries.get_unchecked(i) };
                if entry.key == *key {
                    return Some(i);
                }
            }
            None
        }
    }

    /// Insert a new entry inline (pre-spill only).
    fn insert_inline(&mut self, key: K, value: V) {
        let new_index = self.size as usize;
        
        // Create new entry
        let new_entry = Entry {
            key,
            val: value,
            next: u16::MAX, // Will be set to current tail
            prev: self.tail, // Previous MRU
        };

        // Add to store
        self.store.push(new_entry);

        // Update linked list
        if self.size == 0 {
            // First entry - set as both head and tail
            self.head = new_index as u16;
            self.tail = new_index as u16;
        } else {
            // Link to previous tail
            self.store[self.tail as usize].next = new_index as u16;
            self.tail = new_index as u16;
        }

        // Update size
        self.size += 1;
    }

    /// Promote an entry to MRU (move to tail).
    #[inline(always)]
    fn promote_to_mru(&mut self, index: usize) {
        // Early return if already MRU or only one element
        if self.size <= 1 || index == self.tail as usize {
            return;
        }

        // SAFETY: `index` comes from `find_key_index`, so 0 <= index < self.size.
        // We use unchecked access to eliminate bounds checks on the hot path.
        let entries: *mut [Entry<K, V>] = self.store.as_mut_slice();
        // SAFETY: `index` is guaranteed to be in bounds (0 <= index < self.size) since it
        // comes from `find_key_index`. The raw pointer dereference is safe because `entries`
        // is derived from `self.store.as_mut_slice()` which is a valid mutable slice.
        let entry = unsafe { (&mut *entries).get_unchecked_mut(index) };
        let prev = entry.prev;
        let next = entry.next;
        let entry_index = index as u16;

        // Detach from current position
        if entry_index == self.head {
            // Moving head to somewhere later: head becomes next
            self.head = next;
            if self.head != u16::MAX {
                let head_idx = self.head as usize;
                // SAFETY: `head_idx` is derived from `self.head` which is a valid index into
                // the store (either u16::MAX sentinel or a valid index < self.size). Since we
                // just set `self.head = next` and `next` comes from a valid entry, `head_idx`
                // is guaranteed to be in bounds. The raw pointer dereference is safe as above.
                unsafe { (&mut *entries).get_unchecked_mut(head_idx) }.prev = u16::MAX;
            }
        } else {
            // Update previous node's next to skip this entry
            let prev_idx = prev as usize;
            // SAFETY: `prev_idx` is derived from `prev` which comes from a valid entry's `prev`
            // field. Since we're not at the head (else branch), `prev` must be a valid index
            // < self.size. The raw pointer dereference is safe as above.
            unsafe { (&mut *entries).get_unchecked_mut(prev_idx) }.next = next;
        }

        // Update next node's prev to skip this entry (if it exists)
        if next != u16::MAX {
            let next_idx = next as usize;
            // SAFETY: `next_idx` is derived from `next` which comes from a valid entry's `next`
            // field. Since `next != u16::MAX`, it must be a valid index < self.size. The raw
            // pointer dereference is safe as above.
            unsafe { (&mut *entries).get_unchecked_mut(next_idx) }.prev = prev;
        }

        // Attach at tail
        let old_tail = self.tail;
        entry.prev = old_tail;
        entry.next = u16::MAX;
        let old_tail_idx = old_tail as usize;
        // SAFETY: `old_tail_idx` is derived from `self.tail` which is guaranteed to be a valid
        // index < self.size (not u16::MAX) since we're in the promote_to_mru function and there
        // are at least 2 elements (early return check). The raw pointer dereference is safe as above.
        unsafe { (&mut *entries).get_unchecked_mut(old_tail_idx) }.next = entry_index;
        self.tail = entry_index;
    }

    /// Remove a node from the doubly-linked list.
    fn remove_from_dll(&mut self, _index: usize, prev: u16, next: u16) {
        // Update previous node's next pointer
        if prev != u16::MAX {
            self.store[prev as usize].next = next;
        } else {
            // This was the head - update head
            self.head = next;
        }
        
        // Update next node's prev pointer
        if next != u16::MAX {
            self.store[next as usize].prev = prev;
        } else {
            // This was the tail - update tail
            self.tail = prev;
        }
    }

    /// Update the index of a swapped element in the DLL.
    fn update_swapped_element_index(&mut self, old_index: usize) {
        let new_index = old_index;
        
        // Copy the prev/next values to avoid borrow conflicts
        let prev = self.store[new_index].prev;
        let next = self.store[new_index].next;
        
        // Update references to this element
        if prev != u16::MAX {
            self.store[prev as usize].next = new_index as u16;
        } else {
            // This is now the head
            self.head = new_index as u16;
        }
        
        if next != u16::MAX {
            self.store[next as usize].prev = new_index as u16;
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


