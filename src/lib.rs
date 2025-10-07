#![cfg_attr(not(feature = "std"), no_std)]

//! tiny-lru: A Fast Small-then-Spill LRU cache.
//!
//! This is scaffolding only. All functions are intentionally unimplemented.

extern crate alloc;

use core::hash::Hash;
use tinyvec::TinyVec;
use likely_stable::unlikely;

/// Maximum capacity for v1 implementation (u16::MAX - 1)
const MAX_CAPACITY: u16 = u16::MAX - 1;

/// Intrusive node stored in the TinyVec/heap storage.
#[derive(Default, Clone)]
pub struct Entry<K, V> 
where
    K: Default + Clone,
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
    K: PartialEq + Hash + Default + Clone,
    V: Default,
{
    // Unified node storage; starts inline, spills to heap as capacity grows.
    // Compact: no holes; deletions swap the last element into the freed index.
    store: TinyVec<[Entry<K, V>; N]>,


    // LRU linkage heads (indices into `store`)
    head: u16, // LRU index; sentinel if empty
    tail: u16, // MRU index; sentinel if empty

    // Key → index map. Lazily allocated ONLY on first spill to avoid heap allocs pre-spill.
    // Pre-spill lookups use linear scan over the compact TinyVec.
    index: Option<rustc_hash::FxHashMap<K, u16>>,

    // Capacity semantics (v1 cap):
    // - size and capacity are u16; maximum capacity <= 65,534 (u16::MAX - 1)
    // - set_capacity requires new_cap > size and new_cap >= N
    capacity: u16,
}

// Compile-time assertion: N must be <= MAX_CAPACITY
const fn assert_capacity_limit<const N: usize>() {
    assert!(N <= MAX_CAPACITY as usize, "N must be <= MAX_CAPACITY for v1 capacity limits");
}

impl<K, V, const N: usize> TinyLru<K, V, N>
where
    K: Eq + Hash + Default + Clone,
    V: Default,
{
    /// Create with capacity = N.
    pub fn new() -> Self {
        assert_capacity_limit::<N>();
        
        Self {
            store: TinyVec::new(),
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            index: None,    // No HashMap allocated pre-spill
            capacity: N as u16,
        }
    }

    /// Create with specified capacity (must be >= N).
    pub fn with_capacity(cap: u16) -> Self {
        assert_capacity_limit::<N>();
        
        // Runtime assertion: capacity must be >= N
        assert!(cap >= N as u16, "capacity must be >= N");
        
        Self {
            store: TinyVec::new(),
            head: u16::MAX, // Sentinel value for empty list
            tail: u16::MAX, // Sentinel value for empty list
            index: None,    // No HashMap allocated pre-spill
            capacity: cap,
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

        if unlikely(self.store.len() == N && self.capacity > N as u16) { 
            self.spill();
        }
        if unlikely(self.store.len() >= self.capacity as usize) {
            self.pop();
        }

        self.insert(key, value);
    }

    /// Pop and return the LRU entry.
    pub fn pop(&mut self) -> Option<(K, V)> {
        if self.is_empty() {
            return None;
        }

        // Get the LRU index (head)
        let lru_index = self.head as usize;
        // Capture next before swap_remove
        let next_index_before = self.store[lru_index].next;
        let last_index_before = self.store.len() - 1;
        
        // Remove LRU key from index (if post-spill)
        if self.index.is_some() {
            let lru_key = self.store[lru_index].key.clone();
            self.index.as_mut().unwrap().remove(&lru_key);
        }
        
        // Extract the key-value pair before removal
        let entry = self.store.swap_remove(lru_index);
        let (key, value) = (entry.key, entry.val);
        
        // Handle DLL updates
        if unlikely(self.store.len() == 0) {
            // Last element removed - reset to empty state
            self.head = u16::MAX;
            self.tail = u16::MAX;
        } else {
            // Update head to next element (adjust if it pointed at the old last index)
            if next_index_before as usize != last_index_before {
                self.head = next_index_before;
            }
            self.store[self.head as usize].prev = u16::MAX;
            
            // If we swapped with the last element, update its index in the DLL
            if lru_index < self.store.len() {
                self.update_swapped_element_index(lru_index);
                
                // Update index for the swapped element (if post-spill)
                if self.index.is_some() {
                    // Remove the old key at lru_index (the last element's key)
                    let swapped_key = self.store[lru_index].key.clone();
                    self.index.as_mut().unwrap().remove(&swapped_key);
                    
                    // Insert the swapped element's key at its new position
                    self.index.as_mut().unwrap().insert(swapped_key, lru_index as u16);
                }
            }
        }
        
        Some((key, value))
    }

    /// Get by key, promoting to MRU on hit.
    #[inline]
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&self.store[index].val)
        } else {
            None
        }
    }

    /// Get mutable by key, promoting to MRU on hit.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if let Some(index) = self.find_key_index(key) {
            self.promote_to_mru(index);
            Some(&mut self.store[index].val)
        } else {
            None
        }
    }

    /// Peek without promotion.
    #[inline]
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.find_key_index(key).map(|index| &self.store[index].val)
    }

    /// Remove by key and return owned pair.
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        // Find the key index
        let index = self.find_key_index(key)?;
        
        let last_index_before = self.store.len() - 1;
        let removed_prev = self.store[index].prev;
        let removed_next = self.store[index].next;
        
        // Remove target key from index (if post-spill)
        if self.index.is_some() {
            let target_key = self.store[index].key.clone();
            self.index.as_mut().unwrap().remove(&target_key);
        }
        
        // Extract the key-value pair before removal
        let entry = self.store.swap_remove(index);
        let (key, value) = (entry.key, entry.val);
        
        // Handle DLL updates
        if self.store.len() == 0 {
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
            if index < self.store.len() {
                self.update_swapped_element_index(index);
                
                // Update index for the swapped element (if post-spill)
                if self.index.is_some() {
                    // Remove the old key at index (the last element's key)
                    let swapped_key = self.store[index].key.clone();
                    self.index.as_mut().unwrap().remove(&swapped_key);
                    
                    // Insert the swapped element's key at its new position
                    self.index.as_mut().unwrap().insert(swapped_key, index as u16);
                }
            }
        }
        
        Some((key, value))
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        // Clear the store efficiently
        self.store.clear();
        
        // Reset state to empty
        self.head = u16::MAX; // Sentinel value for empty list
        self.tail = u16::MAX; // Sentinel value for empty list
        
        // Clear HashMap index - reset to pre-spill state
        self.index = None;
    }

    /// Adjust capacity. Requires new_cap > size and new_cap >= N.
    pub fn set_capacity(&mut self, new_cap: u16) {
        // Validate requirements
        assert!(new_cap > self.store.len() as u16, "new_cap must be > current size");
        assert!(new_cap >= N as u16, "new_cap must be >= N");
        
        // Pre-spill: just update the capacity field
        // The TinyVec will handle spill automatically when we exceed N
        self.capacity = new_cap;
    }

    /// Current number of items.
    #[inline]
    pub fn len(&self) -> u16 {
        self.store.len() as u16
    }

    /// Whether the cache is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Current capacity.
    pub fn capacity(&self) -> u16 {
        self.capacity
    }

    /// Contains key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.find_key_index(key).is_some()
    }

    /// Check if the cache is currently spilled to heap.
    pub fn is_spilled(&self) -> bool {
        self.index.is_some()
    }

    /// Check if unspill is currently possible.
    pub fn can_unspill(&self) -> bool {
        self.is_spilled() && self.store.len() <= N
    }

    /// Attempt to unspill from heap back to inline storage.
    /// Returns true if unspill was successful, false if not possible.
    pub fn unspill(&mut self) -> bool {
        if !self.can_unspill() {
            return false;
        }

        // Let tinyvec handle the storage transition
        self.store.shrink_to_fit();

        // Clear HashMap index to return to pre-spill state
        self.index = None;

        true
    }

    /// Find the index of a key.
    /// - Pre-spill: linear scan over compact TinyVec
    /// - Post-spill: O(1) hashmap index lookup
    /// Returns None if key not found.
    #[inline(always)]
    fn find_key_index(&self, key: &K) -> Option<usize> {
        if let Some(index) = &self.index {
            // Post-spill: look up via hashmap index
            index.get(key).map(|&idx| idx as usize)
        } else {
            // Pre-spill: use raw slice iteration
            let entries = &self.store[..self.store.len()];
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

    /// Spill to heap.
    #[cold]
    fn spill(&mut self) {
        self.index = Some(rustc_hash::FxHashMap::default());
        for i in 0..self.store.len() {
            self.index.as_mut().unwrap().insert(self.store[i].key.clone(), i as u16);
        }
    }

    /// Insert a new entry
    #[inline(always)]
    fn insert(&mut self, key: K, value: V) {
        let new_index = self.store.len();
        
        if self.index.is_some() {
            self.index.as_mut().unwrap().insert(key.clone(), new_index as u16);
        }

        // Create new entry
        let new_entry = Entry {
            key: key, // Clone for the entry
            val: value,
            next: u16::MAX, // Will be set to current tail
            prev: self.tail, // Previous MRU
        };

        // Add to store
        self.store.push(new_entry);

        // Update linked list
        if self.tail == u16::MAX {
            // First entry - set as both head and tail
            self.head = new_index as u16;
            self.tail = new_index as u16;
        } else {
            // Link to previous tail
            self.store[self.tail as usize].next = new_index as u16;
            self.tail = new_index as u16;
        }

    }

    /// Promote an entry to MRU (move to tail).
    #[inline(always)]
    fn promote_to_mru(&mut self, index: usize) {
        // Early return if already MRU or only one element
        if self.store.len() <= 1 || index == self.tail as usize {
            return;
        }

        // SAFETY: `index` comes from `find_key_index`, so 0 <= index < self.store.len().
        // We use unchecked access to eliminate bounds checks on the hot path.
        let entries: *mut [Entry<K, V>] = self.store.as_mut_slice();
        // SAFETY: `index` is guaranteed to be in bounds (0 <= index < self.store.len()) since it
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
                // the store (either u16::MAX sentinel or a valid index < self.store.len()). Since we
                // just set `self.head = next` and `next` comes from a valid entry, `head_idx`
                // is guaranteed to be in bounds. The raw pointer dereference is safe as above.
                unsafe { (&mut *entries).get_unchecked_mut(head_idx) }.prev = u16::MAX;
            }
        } else {
            // Update previous node's next to skip this entry
            let prev_idx = prev as usize;
            // SAFETY: `prev_idx` is derived from `prev` which comes from a valid entry's `prev`
            // field. Since we're not at the head (else branch), `prev` must be a valid index
            // < self.store.len(). The raw pointer dereference is safe as above.
            unsafe { (&mut *entries).get_unchecked_mut(prev_idx) }.next = next;
        }

        // Update next node's prev to skip this entry (if it exists)
        if next != u16::MAX {
            let next_idx = next as usize;
            // SAFETY: `next_idx` is derived from `next` which comes from a valid entry's `next`
            // field. Since `next != u16::MAX`, it must be a valid index < self.store.len(). The raw
            // pointer dereference is safe as above.
            unsafe { (&mut *entries).get_unchecked_mut(next_idx) }.prev = prev;
        }

        // Attach at tail
        let old_tail = self.tail;
        entry.prev = old_tail;
        entry.next = u16::MAX;
        let old_tail_idx = old_tail as usize;
        // SAFETY: `old_tail_idx` is derived from `self.tail` which is guaranteed to be a valid
        // index < self.store.len() (not u16::MAX) since we're in the promote_to_mru function and there
        // are at least 2 elements (early return check). The raw pointer dereference is safe as above.
        unsafe { (&mut *entries).get_unchecked_mut(old_tail_idx) }.next = entry_index;
        self.tail = entry_index;
    }

    /// Remove a node from the doubly-linked list.
    #[inline(always)]
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
    #[inline(always)]
    fn update_swapped_element_index(&mut self, new_index: usize) {
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


