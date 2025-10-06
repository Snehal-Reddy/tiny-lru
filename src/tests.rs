use super::*;
use core::hash::Hash;

// Helper function to create a cache with some test data
fn create_test_cache() -> TinyLru<&'static str, i32, 4> {
    let mut cache = TinyLru::new();
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache
}

// Helper function to verify DLL structure
fn verify_dll_structure<K: Eq + Hash + Default + Clone, V: Default, const N: usize>(cache: &TinyLru<K, V, N>) {
    if cache.is_empty() {
        assert_eq!(cache.head, u16::MAX);
        assert_eq!(cache.tail, u16::MAX);
        return;
    }

    // Verify head and tail are valid
    assert!(cache.head < cache.len());
    assert!(cache.tail < cache.len());
    
    // Verify head has no previous element
    assert_eq!(cache.store[cache.head as usize].prev, u16::MAX);
    
    // Verify tail has no next element
    assert_eq!(cache.store[cache.tail as usize].next, u16::MAX);
    
    // Verify DLL chain integrity
    let mut current = cache.head;
    let mut count = 0;
    
    while current != u16::MAX {
        count += 1;
        let entry = &cache.store[current as usize];
        
        // Verify next->prev points back to current
        if entry.next != u16::MAX {
            assert_eq!(cache.store[entry.next as usize].prev, current);
        }
        
        // Verify prev->next points forward to current
        if entry.prev != u16::MAX {
            assert_eq!(cache.store[entry.prev as usize].next, current);
        }
        
        current = entry.next;
    }
    
    // Verify we visited all elements
    assert_eq!(count, cache.len());
}

#[test]
fn test_new() {
    let cache: TinyLru<&str, i32, 4> = TinyLru::new();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 4);
    assert!(!cache.index.is_some());
    verify_dll_structure(&cache);
}

#[test]
fn test_with_capacity() {
    let cache: TinyLru<&str, i32, 4> = TinyLru::with_capacity(8);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 8);
    assert!(!cache.index.is_some());
    verify_dll_structure(&cache);
}

#[test]
#[should_panic(expected = "capacity must be >= N")]
fn test_with_capacity_invalid() {
    let _cache: TinyLru<&str, i32, 4> = TinyLru::with_capacity(2);
}

#[test]
fn test_push_single() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("key", 42);
    
    assert_eq!(cache.len(), 1);
    assert!(!cache.is_empty());
    assert_eq!(cache.capacity(), 4);
    assert!(!cache.index.is_some());
    verify_dll_structure(&cache);
}

#[test]
fn test_push_multiple() {
    let cache = create_test_cache();
    
    assert_eq!(cache.len(), 3);
    assert!(!cache.is_empty());
    assert!(!cache.index.is_some());
    verify_dll_structure(&cache);
}

#[test]
fn test_push_update_existing() {
    let mut cache = create_test_cache();
    
    // Update existing key
    cache.push("b", 99);
    
    assert_eq!(cache.len(), 3); // Size should remain the same
    assert_eq!(cache.get(&"b"), Some(&99));
    verify_dll_structure(&cache);
}

#[test]
fn test_get_existing() {
    let mut cache = create_test_cache();
    
    // Get existing key
    assert_eq!(cache.get(&"a"), Some(&1));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_get_nonexistent() {
    let mut cache = create_test_cache();
    
    // Get non-existent key
    assert_eq!(cache.get(&"nonexistent"), None);
    
    verify_dll_structure(&cache);
}

#[test]
fn test_get_mut_existing() {
    let mut cache = create_test_cache();
    
    // Get mutable reference to existing key
    if let Some(val) = cache.get_mut(&"b") {
        *val = 99;
    }
    
    assert_eq!(cache.get(&"b"), Some(&99));
    verify_dll_structure(&cache);
}

#[test]
fn test_get_mut_nonexistent() {
    let mut cache = create_test_cache();
    
    // Get mutable reference to non-existent key
    assert_eq!(cache.get_mut(&"nonexistent"), None);
    
    verify_dll_structure(&cache);
}

#[test]
fn test_peek_existing() {
    let cache = create_test_cache();
    
    // Peek at existing key (should not promote)
    assert_eq!(cache.peek(&"a"), Some(&1));
    assert_eq!(cache.peek(&"b"), Some(&2));
    assert_eq!(cache.peek(&"c"), Some(&3));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_peek_nonexistent() {
    let cache = create_test_cache();
    
    // Peek at non-existent key
    assert_eq!(cache.peek(&"nonexistent"), None);
    
    verify_dll_structure(&cache);
}

#[test]
fn test_contains_key() {
    let cache = create_test_cache();
    
    assert!(cache.contains_key(&"a"));
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(!cache.contains_key(&"nonexistent"));
}

#[test]
fn test_remove_existing() {
    let mut cache = create_test_cache();
    
    // Remove existing key
    let removed = cache.remove(&"b");
    assert_eq!(removed, Some(("b", 2)));
    assert_eq!(cache.len(), 2);
    assert!(!cache.contains_key(&"b"));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_remove_nonexistent() {
    let mut cache = create_test_cache();
    
    // Remove non-existent key
    let removed = cache.remove(&"nonexistent");
    assert_eq!(removed, None);
    assert_eq!(cache.len(), 3); // Size should remain unchanged
    
    verify_dll_structure(&cache);
}

#[test]
fn test_remove_single() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("only", 42);
    
    let removed = cache.remove(&"only");
    assert_eq!(removed, Some(("only", 42)));
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    
    verify_dll_structure(&cache);
}

#[test]
fn test_pop_empty() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    
    let popped = cache.pop();
    assert_eq!(popped, None);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    
    verify_dll_structure(&cache);
}

#[test]
fn test_pop_single() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("only", 42);
    
    let popped = cache.pop();
    assert_eq!(popped, Some(("only", 42)));
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    
    verify_dll_structure(&cache);
}

#[test]
fn test_pop_multiple() {
    let mut cache = create_test_cache();
    
    // Pop should return LRU (first inserted)
    let popped = cache.pop();
    assert_eq!(popped, Some(("a", 1)));
    assert_eq!(cache.len(), 2);
    assert!(!cache.contains_key(&"a"));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_clear() {
    let mut cache = create_test_cache();
    
    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 4); // Capacity should remain unchanged
    assert!(!cache.index.is_some());
    
    verify_dll_structure(&cache);
}

#[test]
fn test_set_capacity_valid() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("a", 1);
    cache.push("b", 2);
    
    // Increase capacity
    cache.set_capacity(8);
    assert_eq!(cache.capacity(), 8);
    assert_eq!(cache.len(), 2);
    
    verify_dll_structure(&cache);
}

#[test]
#[should_panic(expected = "new_cap must be > current size")]
fn test_set_capacity_too_small() {
    let mut cache = create_test_cache();
    cache.set_capacity(2); // Less than current size (3)
}

#[test]
#[should_panic(expected = "new_cap must be >= N")]
fn test_set_capacity_below_n() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.set_capacity(2); // Less than N (4)
}

#[test]
fn test_lru_promotion_on_get() {
    let mut cache = create_test_cache();
    
    // Get "a" (LRU) - should promote to MRU
    assert_eq!(cache.get(&"a"), Some(&1));
    
    // Now "b" should be LRU
    let popped = cache.pop();
    assert_eq!(popped, Some(("b", 2)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_lru_promotion_on_get_mut() {
    let mut cache = create_test_cache();
    
    // Get mutable reference to "a" (LRU) - should promote to MRU
    if let Some(val) = cache.get_mut(&"a") {
        *val = 99;
    }
    
    // Now "b" should be LRU
    let popped = cache.pop();
    assert_eq!(popped, Some(("b", 2)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_no_promotion_on_peek() {
    let mut cache = create_test_cache();
    
    // Peek at "a" (LRU) - should NOT promote
    assert_eq!(cache.peek(&"a"), Some(&1));
    
    // "a" should still be LRU
    let popped = cache.pop();
    assert_eq!(popped, Some(("a", 1)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_eviction_when_full() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::new();
    
    // Fill cache to capacity
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    
    assert_eq!(cache.len(), 3);
    assert_eq!(cache.capacity(), 3);
    
    // Add one more - should evict LRU ("a")
    cache.push("d", 4);
    
    assert_eq!(cache.len(), 3);
    assert!(!cache.contains_key(&"a"));
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_swap_remove_behavior() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4);
    
    // Remove middle element ("b")
    let removed = cache.remove(&"b");
    assert_eq!(removed, Some(("b", 2)));
    assert_eq!(cache.len(), 3);
    
    // Verify all remaining elements are still accessible
    assert!(cache.contains_key(&"a"));
    assert!(!cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_swap_remove_head() {
    let mut cache = create_test_cache();
    
    // Remove head element ("a")
    let removed = cache.remove(&"a");
    assert_eq!(removed, Some(("a", 1)));
    assert_eq!(cache.len(), 2);
    
    // "b" should now be head
    let popped = cache.pop();
    assert_eq!(popped, Some(("b", 2)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_swap_remove_tail() {
    let mut cache = create_test_cache();
    
    // Remove tail element ("c")
    let removed = cache.remove(&"c");
    assert_eq!(removed, Some(("c", 3)));
    assert_eq!(cache.len(), 2);
    
    // "b" should now be tail
    assert_eq!(cache.get(&"b"), Some(&2));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_complex_operations() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    
    // Insert elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    
    // Update existing
    cache.push("b", 99);
    
    // Promote via get
    assert_eq!(cache.get(&"a"), Some(&1));
    
    // Remove middle
    cache.remove(&"c");
    
    // Add new
    cache.push("d", 4);
    
    // Verify final state
    assert_eq!(cache.len(), 3);
    assert!(cache.contains_key(&"a"));
    assert!(cache.contains_key(&"b"));
    assert!(!cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    
    // "b" should be LRU (was updated but not accessed)
    let popped = cache.pop();
    assert_eq!(popped, Some(("b", 99)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_different_key_types() {
    let mut cache: TinyLru<i32, &str, 4> = TinyLru::new();
    
    cache.push(1, "one");
    cache.push(2, "two");
    cache.push(3, "three");
    
    assert_eq!(cache.get(&1), Some(&"one"));
    assert_eq!(cache.get(&2), Some(&"two"));
    assert_eq!(cache.get(&3), Some(&"three"));
    
    let removed = cache.remove(&2);
    assert_eq!(removed, Some((2, "two")));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_different_value_types() {
    let mut cache: TinyLru<&str, Vec<i32>, 4> = TinyLru::new();
    
    cache.push("vec1", vec![1, 2, 3]);
    cache.push("vec2", vec![4, 5, 6]);
    
    assert_eq!(cache.get(&"vec1"), Some(&vec![1, 2, 3]));
    assert_eq!(cache.get(&"vec2"), Some(&vec![4, 5, 6]));
    
    // Modify via get_mut
    if let Some(vec) = cache.get_mut(&"vec1") {
        vec.push(4);
    }
    
    assert_eq!(cache.get(&"vec1"), Some(&vec![1, 2, 3, 4]));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_capacity_edge_cases() {
    // Test with N = 1
    let mut cache: TinyLru<&str, i32, 1> = TinyLru::new();
    assert_eq!(cache.capacity(), 1);
    
    cache.push("a", 1);
    assert_eq!(cache.len(), 1);
    
    // Adding another should evict "a"
    cache.push("b", 2);
    assert_eq!(cache.len(), 1);
    assert!(!cache.contains_key(&"a"));
    assert!(cache.contains_key(&"b"));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_sentinel_values() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    
    // Empty cache should have sentinel values
    assert_eq!(cache.head, u16::MAX);
    assert_eq!(cache.tail, u16::MAX);
    
    // Add one element
    cache.push("a", 1);
    assert_ne!(cache.head, u16::MAX);
    assert_ne!(cache.tail, u16::MAX);
    assert_eq!(cache.head, cache.tail); // Single element is both head and tail
    
    // Remove element
    cache.remove(&"a");
    assert_eq!(cache.head, u16::MAX);
    assert_eq!(cache.tail, u16::MAX);
    
    verify_dll_structure(&cache);
}

#[test]
fn test_promotion_edge_cases() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    
    // Single element - promotion should be no-op
    cache.push("a", 1);
    cache.promote_to_mru(0);
    assert_eq!(cache.head, 0);
    assert_eq!(cache.tail, 0);
    
    // Two elements - promote head to tail
    cache.push("b", 2);
    assert_eq!(cache.head, 0); // "a" is head
    assert_eq!(cache.tail, 1); // "b" is tail
    
    cache.promote_to_mru(0); // Promote "a"
    assert_eq!(cache.head, 1); // "b" is now head
    assert_eq!(cache.tail, 0); // "a" is now tail
    
    verify_dll_structure(&cache);
}

#[test]
fn test_remove_all_elements() {
    let mut cache = create_test_cache();
    
    // Remove all elements one by one
    assert_eq!(cache.remove(&"a"), Some(("a", 1)));
    assert_eq!(cache.len(), 2);
    
    assert_eq!(cache.remove(&"b"), Some(("b", 2)));
    assert_eq!(cache.len(), 1);
    
    assert_eq!(cache.remove(&"c"), Some(("c", 3)));
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    
    verify_dll_structure(&cache);
}

#[test]
fn test_pop_all_elements() {
    let mut cache = create_test_cache();
    
    // Pop all elements one by one
    assert_eq!(cache.pop(), Some(("a", 1)));
    assert_eq!(cache.len(), 2);
    
    assert_eq!(cache.pop(), Some(("b", 2)));
    assert_eq!(cache.len(), 1);
    
    assert_eq!(cache.pop(), Some(("c", 3)));
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    
    verify_dll_structure(&cache);
}

// ============================================================================
// POST-SPILL TESTS
// ============================================================================

// Helper function to force a cache into post-spill state
fn force_spill(cache: &mut TinyLru<&str, i32, 3>) {
    // Fill to exactly N elements, then add one more to trigger spill
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
}

// Helper function to verify index consistency post-spill
fn verify_index_consistency<K: Eq + Hash + Default + Clone, V: Default, const N: usize>(
    cache: &TinyLru<K, V, N>
) {
    if cache.index.is_none() {
        return; // Pre-spill, no index to verify
    }
    
    let index = cache.index.as_ref().unwrap();
    
    // Verify all entries in the array have corresponding index entries
    for i in 0..cache.len() as usize {
        let key = &cache.store[i].key;
        assert_eq!(index.get(key), Some(&(i as u16)), 
                   "Index entry for key at position {} should point to index {}", i, i);
    }
    
    // Verify index size matches array size
    assert_eq!(index.len(), cache.len() as usize,
               "Index size {} should match array size {}", index.len(), cache.len());
}

// Helper function to verify post-spill state
fn verify_post_spill_state<K: Eq + Hash + Default + Clone, V: Default, const N: usize>(
    cache: &TinyLru<K, V, N>
) {
    assert!(cache.index.is_some(), "Cache should be in post-spill state");
    verify_index_consistency(cache);
    verify_dll_structure(cache);
}

// ============================================================================
// SPILL TRANSITION TESTS
// ============================================================================

#[test]
fn test_basic_spill_transition() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Pre-spill: should have no index
    assert!(cache.index.is_none());
    
    // Fill to exactly N elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    
    // Still pre-spill
    assert!(cache.index.is_none());
    assert_eq!(cache.len(), 3);
    
    // Add one more element to trigger spill
    cache.push("d", 4);
    
    // Now post-spill
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 4);
}

#[test]
fn test_spill_with_different_n_values() {
    // Test with N=1
    let mut cache1: TinyLru<&str, i32, 1> = TinyLru::with_capacity(2);
    cache1.push("a", 1);
    assert!(cache1.index.is_none()); // Still pre-spill
    cache1.push("b", 2); // Triggers spill
    verify_post_spill_state(&cache1);
    
    // Test with N=2
    let mut cache2: TinyLru<&str, i32, 2> = TinyLru::with_capacity(3);
    cache2.push("a", 1);
    cache2.push("b", 2);
    assert!(cache2.index.is_none()); // Still pre-spill
    cache2.push("c", 3); // Triggers spill
    verify_post_spill_state(&cache2);
    
    // Test with N=4
    let mut cache4: TinyLru<String, i32, 4> = TinyLru::with_capacity(5);
    for i in 0..4 {
        let key = format!("key{}", i);
        cache4.push(key, i);
    }
    assert!(cache4.index.is_none()); // Still pre-spill
    cache4.push("key4".to_string(), 4); // Triggers spill
    verify_post_spill_state(&cache4);
}

#[test]
fn test_spill_with_capacity_larger_than_n() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(8);
    
    // Fill to N elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    assert!(cache.index.is_none()); // Still pre-spill
    
    // Add one more to trigger spill
    cache.push("d", 4);
    verify_post_spill_state(&cache);
    assert_eq!(cache.capacity(), 8);
    assert_eq!(cache.len(), 4);
}

#[test]
fn test_spill_state_persistence() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Remove elements to get below N
    cache.pop(); // Remove one element
    assert_eq!(cache.len(), 3);
    assert!(cache.index.is_some(), "Index should persist even with fewer than N elements");
    
    // Remove more elements
    cache.pop();
    cache.pop();
    assert_eq!(cache.len(), 1);
    assert!(cache.index.is_some(), "Index should persist even with just 1 element");
    
    // Clear and verify index is cleared
    cache.clear();
    assert!(cache.index.is_none(), "Index should be cleared after clear()");
}

// ============================================================================
// POST-SPILL PUSH OPERATIONS TESTS
// ============================================================================

#[test]
fn test_push_new_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Push new key post-spill
    cache.push("new_key", 100);
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 4); // At capacity, should evict LRU
    assert_eq!(cache.get(&"new_key"), Some(&100));
    // "a" should be evicted (was LRU)
    assert_eq!(cache.get(&"a"), None);
}

#[test]
fn test_push_existing_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Update existing key post-spill
    cache.push("b", 200);
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 4); // Size unchanged
    assert_eq!(cache.get(&"b"), Some(&200));
    
    // Verify "b" is now MRU (last in DLL)
    assert_eq!(cache.tail, cache.find_key_index(&"b").unwrap() as u16);
}

#[test]
fn test_push_at_capacity_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(5);
    
    // Fill to capacity
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    cache.push("e", 5); // At capacity
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 5);
    assert_eq!(cache.capacity(), 5);
    
    // Push one more - should evict LRU
    cache.push("f", 6);
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 5); // Size unchanged
    assert_eq!(cache.capacity(), 5);
    
    // "a" should be evicted (was LRU)
    assert_eq!(cache.get(&"a"), None);
    assert_eq!(cache.get(&"f"), Some(&6));
}

#[test]
fn test_multiple_pushes_post_spill() {
    let mut cache: TinyLru<String, i32, 2> = TinyLru::with_capacity(3);
    
    // Trigger spill
    cache.push("a".to_string(), 1);
    cache.push("b".to_string(), 2);
    cache.push("c".to_string(), 3); // Triggers spill
    verify_post_spill_state(&cache);
    
    // Multiple pushes post-spill
    for i in 0..10 {
        let key = format!("key{}", i);
        cache.push(key, i);
        verify_post_spill_state(&cache);
    }
    
    // Should have evicted some elements
    assert_eq!(cache.len(), 3); // N+1 elements
    assert!(cache.get(&"key0".to_string()).is_none()); // Should be evicted
    assert!(cache.get(&"key9".to_string()).is_some()); // Should be present (MRU)
}

// ============================================================================
// POST-SPILL GET OPERATIONS TESTS
// ============================================================================

#[test]
fn test_get_existing_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Get existing key post-spill
    assert_eq!(cache.get(&"b"), Some(&2));
    verify_post_spill_state(&cache);
    
    // Verify "b" is promoted to MRU
    assert_eq!(cache.tail, cache.find_key_index(&"b").unwrap() as u16);
}

#[test]
fn test_get_nonexistent_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Get non-existent key post-spill
    assert_eq!(cache.get(&"nonexistent"), None);
    verify_post_spill_state(&cache);
}

#[test]
fn test_get_mut_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Get mutable reference post-spill
    if let Some(value) = cache.get_mut(&"b") {
        *value = 200;
    }
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.get(&"b"), Some(&200));
    
    // Verify "b" is promoted to MRU
    assert_eq!(cache.tail, cache.find_key_index(&"b").unwrap() as u16);
}

#[test]
fn test_peek_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Peek existing key post-spill (should not promote)
    let initial_tail = cache.tail;
    assert_eq!(cache.peek(&"b"), Some(&2));
    assert_eq!(cache.tail, initial_tail); // Tail should not change
    
    // Peek non-existent key post-spill
    assert_eq!(cache.peek(&"nonexistent"), None);
}

// ============================================================================
// POST-SPILL REMOVE OPERATIONS TESTS
// ============================================================================

#[test]
fn test_remove_existing_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Remove existing key post-spill
    assert_eq!(cache.remove(&"b"), Some(("b", 2)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 3);
    assert_eq!(cache.get(&"b"), None);
}

#[test]
fn test_remove_nonexistent_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Remove non-existent key post-spill
    assert_eq!(cache.remove(&"nonexistent"), None);
    verify_post_spill_state(&cache);
}

#[test]
fn test_remove_head_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Remove head (LRU) post-spill
    let initial_head = cache.head;
    assert_eq!(cache.remove(&"a"), Some(("a", 1)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 3);
    
    // Head should be updated
    assert_ne!(cache.head, initial_head);
}

#[test]
fn test_remove_tail_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Remove tail (MRU) post-spill
    let initial_tail = cache.tail;
    assert_eq!(cache.remove(&"d"), Some(("d", 4)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 3);
    
    // Tail should be updated
    assert_ne!(cache.tail, initial_tail);
}

#[test]
fn test_remove_middle_element_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Remove middle element post-spill
    assert_eq!(cache.remove(&"b"), Some(("b", 2)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 3);
    
    // DLL should still be intact
    verify_dll_structure(&cache);
}

#[test]
fn test_remove_last_element_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with single element post-spill
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3); // Fill to N elements
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 4);
    
    // Remove elements to get to single element
    cache.pop(); // Remove "a"
    cache.pop(); // Remove "b"
    cache.pop(); // Remove "c"
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 1);
    
    // Remove the last element
    assert_eq!(cache.remove(&"d"), Some(("d", 4)));
    assert!(cache.is_empty());
    assert!(cache.index.is_some()); // Index should persist
}

// ============================================================================
// POST-SPILL POP OPERATIONS TESTS
// ============================================================================

#[test]
fn test_pop_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Pop LRU post-spill
    assert_eq!(cache.pop(), Some(("a", 1)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 3);
    
    // Next pop should remove "b"
    assert_eq!(cache.pop(), Some(("b", 2)));
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_pop_last_element_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with single element post-spill
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3); // Fill to N elements
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 4);
    
    // Pop elements to get to single element
    cache.pop(); // Remove "a"
    cache.pop(); // Remove "b" 
    cache.pop(); // Remove "c"
    
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 1);
    
    // Pop the last element
    assert_eq!(cache.pop(), Some(("d", 4)));
    assert!(cache.is_empty());
    assert!(cache.index.is_some()); // Index should persist (not cleared by pop)
}

#[test]
fn test_pop_all_elements_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Pop all elements
    while !cache.is_empty() {
        cache.pop();
        if !cache.is_empty() {
            verify_post_spill_state(&cache);
        }
    }
    
    assert!(cache.is_empty());
    assert!(cache.index.is_some()); // Index should persist (not cleared by pop)
}

// ============================================================================
// INDEX CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_index_consistency_after_swap_remove() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(5);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    cache.push("e", 5);
    
    verify_post_spill_state(&cache);
    
    // Remove middle element (should trigger swap_remove)
    assert_eq!(cache.remove(&"b"), Some(("b", 2)));
    verify_post_spill_state(&cache);
    
    // Verify all remaining elements have correct index entries
    verify_index_consistency(&cache);
}

#[test]
fn test_index_consistency_after_promotion() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Promote "a" to MRU
    cache.get(&"a");
    verify_post_spill_state(&cache);
    
    // Verify index consistency after promotion
    verify_index_consistency(&cache);
}

#[test]
fn test_index_consistency_after_eviction() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Fill to capacity
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Add one more to trigger eviction
    cache.push("e", 5);
    verify_post_spill_state(&cache);
    
    // Verify index consistency after eviction
    verify_index_consistency(&cache);
    
    // "a" should be evicted
    assert_eq!(cache.get(&"a"), None);
}

// ============================================================================
// EDGE CASES AND MIXED OPERATIONS TESTS
// ============================================================================

#[test]
fn test_spill_with_single_element() {
    let mut cache: TinyLru<&str, i32, 1> = TinyLru::with_capacity(2);
    
    // Add first element
    cache.push("a", 1);
    assert!(cache.index.is_none()); // Still pre-spill
    
    // Add second element to trigger spill
    cache.push("b", 2);
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 2);
}

#[test]
fn test_complex_operation_sequence_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Complex sequence of operations
    cache.push("new1", 100);
    verify_post_spill_state(&cache);
    
    cache.get(&"new1"); // Promote to MRU
    verify_post_spill_state(&cache);
    
    cache.remove(&"new1");
    verify_post_spill_state(&cache);
    
    cache.push("new2", 200);
    verify_post_spill_state(&cache);
    
    cache.pop(); // Remove LRU
    verify_post_spill_state(&cache);
    
    // Final state should be consistent
    verify_index_consistency(&cache);
}

#[test]
fn test_contains_key_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Create cache with known elements
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    cache.push("d", 4); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Test contains_key post-spill
    assert!(cache.contains_key(&"a"));
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    assert!(!cache.contains_key(&"nonexistent"));
}

#[test]
fn test_capacity_operations_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Test capacity operations post-spill
    assert_eq!(cache.capacity(), 4);
    assert_eq!(cache.len(), 4);
    
    // Set new capacity
    cache.set_capacity(8);
    verify_post_spill_state(&cache);
    assert_eq!(cache.capacity(), 8);
    assert_eq!(cache.len(), 4);
}

#[test]
fn test_clear_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    verify_post_spill_state(&cache);
    
    // Clear post-spill
    cache.clear();
    assert!(cache.is_empty());
    assert!(cache.index.is_none()); // Index should be cleared
    assert_eq!(cache.capacity(), 4);
}

#[test]
fn test_different_key_types_post_spill() {
    let mut cache: TinyLru<i32, &str, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill with integer keys
    cache.push(1, "a");
    cache.push(2, "b");
    cache.push(3, "c");
    cache.push(4, "d"); // Triggers spill
    
    verify_post_spill_state(&cache);
    
    // Test operations with integer keys
    assert_eq!(cache.get(&2), Some(&"b"));
    assert_eq!(cache.remove(&3), Some((3, "c")));
    verify_post_spill_state(&cache);
}

#[test]
fn test_stress_test_post_spill() {
    let mut cache: TinyLru<String, i32, 4> = TinyLru::with_capacity(5);
    
    // Trigger spill
    cache.push("a".to_string(), 1);
    cache.push("b".to_string(), 2);
    cache.push("c".to_string(), 3);
    cache.push("d".to_string(), 4);
    cache.push("e".to_string(), 5); // Triggers spill
    verify_post_spill_state(&cache);
    
    // Stress test with many operations
    for i in 0..100 {
        let key = format!("key{}", i);
        cache.push(key, i);
        if i % 10 == 0 {
            verify_post_spill_state(&cache);
        }
    }
    
    // Final verification
    verify_post_spill_state(&cache);
    assert_eq!(cache.len(), 5); // N+1 elements
}

// ============================================================================
// UNSPILL TESTS
// ============================================================================

#[test]
fn test_is_spilled_pre_spill() {
    let cache: TinyLru<&str, i32, 4> = TinyLru::new();
    assert!(!cache.is_spilled());
    
    let mut cache = cache;
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    assert!(!cache.is_spilled()); // Still pre-spill
}

#[test]
fn test_is_spilled_post_spill() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
}

#[test]
fn test_can_unspill_pre_spill() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    assert!(!cache.can_unspill()); // Not spilled
    
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    assert!(!cache.can_unspill()); // Still not spilled
}

#[test]
fn test_can_unspill_post_spill_size_too_large() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    assert!(!cache.can_unspill()); // Size (4) > N (3)
}

#[test]
fn test_can_unspill_post_spill_size_fits() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    assert!(!cache.can_unspill()); // Size (4) > N (3)
    
    // Remove one element to make size <= N
    cache.pop();
    assert!(cache.is_spilled());
    assert!(cache.can_unspill()); // Size (3) <= N (3)
}

#[test]
fn test_unspill_pre_spill() {
    let mut cache: TinyLru<&str, i32, 4> = TinyLru::new();
    cache.push("a", 1);
    cache.push("b", 2);
    cache.push("c", 3);
    
    // Try to unspill when not spilled
    assert!(!cache.unspill());
    assert!(!cache.is_spilled());
    assert_eq!(cache.len(), 3);
}

#[test]
fn test_unspill_post_spill_size_too_large() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    assert_eq!(cache.len(), 4);
    
    // Try to unspill when size > N
    assert!(!cache.unspill());
    assert!(cache.is_spilled()); // Should still be spilled
    assert_eq!(cache.len(), 4);
}

#[test]
fn test_unspill_successful() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    assert_eq!(cache.len(), 4);
    
    // Remove one element to make size <= N
    cache.pop();
    assert!(cache.is_spilled());
    assert_eq!(cache.len(), 3);
    assert!(cache.can_unspill());
    
    // Unspill should succeed
    assert!(cache.unspill());
    assert!(!cache.is_spilled()); // Should be back to pre-spill
    assert_eq!(cache.len(), 3);
    
    // Verify all elements are still accessible
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    assert!(!cache.contains_key(&"a")); // Was popped
    
    // Verify DLL structure is intact
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_with_single_element() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Remove all but one element
    cache.pop(); // Remove "a"
    cache.pop(); // Remove "b"
    cache.pop(); // Remove "c"
    assert_eq!(cache.len(), 1);
    assert!(cache.is_spilled());
    assert!(cache.can_unspill());
    
    // Unspill should succeed
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    assert_eq!(cache.len(), 1);
    
    // Verify the single element is still accessible
    assert!(cache.contains_key(&"d"));
    assert_eq!(cache.get(&"d"), Some(&4));
    
    // Verify DLL structure is intact
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_with_two_elements() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Remove two elements to get size = 2
    cache.pop(); // Remove "a"
    cache.pop(); // Remove "b"
    assert_eq!(cache.len(), 2);
    assert!(cache.is_spilled());
    assert!(cache.can_unspill());
    
    // Unspill should succeed
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    assert_eq!(cache.len(), 2);
    
    // Verify both elements are still accessible
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
    
    // Verify DLL structure is intact
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_preserves_lru_order() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Remove one element to make size <= N
    cache.pop(); // Remove "a"
    assert_eq!(cache.len(), 3);
    
    // Promote "b" to MRU before unspill
    cache.get(&"b");
    assert_eq!(cache.tail, cache.find_key_index(&"b").unwrap() as u16);
    
    // Unspill
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    
    // Verify LRU order is preserved
    // "c" should be LRU (head), "b" should be MRU (tail)
    assert_eq!(cache.head, cache.find_key_index(&"c").unwrap() as u16);
    assert_eq!(cache.tail, cache.find_key_index(&"b").unwrap() as u16);
    
    // Verify pop still returns LRU
    let popped = cache.pop();
    assert_eq!(popped, Some(("c", 3)));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_after_operations() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Perform various operations post-spill
    cache.get(&"b"); // Promote "b" to MRU
    cache.push("e", 5); // Add new element (should evict "a")
    cache.remove(&"c"); // Remove "c"
    
    assert_eq!(cache.len(), 3);
    assert!(cache.is_spilled());
    assert!(cache.can_unspill());
    
    // Unspill should succeed
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    assert_eq!(cache.len(), 3);
    
    // Verify final state
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"d"));
    assert!(cache.contains_key(&"e"));
    assert!(!cache.contains_key(&"a")); // Was evicted
    assert!(!cache.contains_key(&"c")); // Was removed
    
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_with_different_n_values() {
    // Test with N=1
    let mut cache1: TinyLru<&str, i32, 1> = TinyLru::with_capacity(2);
    cache1.push("a", 1);
    cache1.push("b", 2); // Triggers spill
    assert!(cache1.is_spilled());
    assert!(!cache1.can_unspill()); // Size (2) > N (1)
    
    cache1.pop(); // Remove one element
    assert!(cache1.can_unspill()); // Size (1) <= N (1)
    assert!(cache1.unspill());
    assert!(!cache1.is_spilled());
    
    // Test with N=2
    let mut cache2: TinyLru<&str, i32, 2> = TinyLru::with_capacity(3);
    cache2.push("a", 1);
    cache2.push("b", 2);
    cache2.push("c", 3); // Triggers spill
    assert!(cache2.is_spilled());
    assert!(!cache2.can_unspill()); // Size (3) > N (2)
    
    cache2.pop(); // Remove one element
    assert!(cache2.can_unspill()); // Size (2) <= N (2)
    assert!(cache2.unspill());
    assert!(!cache2.is_spilled());
}

#[test]
fn test_unspill_after_clear() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Clear the cache
    cache.clear();
    assert!(!cache.is_spilled()); // Clear resets to pre-spill state
    assert!(!cache.can_unspill()); // Can't unspill when not spilled
    
    // Try to unspill after clear
    assert!(!cache.unspill());
    assert!(!cache.is_spilled());
}

#[test]
fn test_unspill_with_capacity_changes() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Change capacity
    cache.set_capacity(8);
    assert!(cache.is_spilled()); // Still spilled
    assert!(!cache.can_unspill()); // Size (4) > N (3)
    
    // Remove one element
    cache.pop();
    assert!(cache.can_unspill()); // Size (3) <= N (3)
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
}

#[test]
fn test_multiple_unspill_attempts() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // First unspill attempt should fail (size > N)
    assert!(!cache.unspill());
    assert!(cache.is_spilled());
    
    // Remove one element
    cache.pop();
    assert!(cache.can_unspill());
    
    // Second unspill attempt should succeed
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    
    // Third unspill attempt should fail (not spilled)
    assert!(!cache.unspill());
    assert!(!cache.is_spilled());
}

#[test]
fn test_unspill_stress_test() {
    let mut cache: TinyLru<String, i32, 4> = TinyLru::with_capacity(5);
    
    // Trigger spill
    cache.push("a".to_string(), 1);
    cache.push("b".to_string(), 2);
    cache.push("c".to_string(), 3);
    cache.push("d".to_string(), 4);
    cache.push("e".to_string(), 5); // Triggers spill
    assert!(cache.is_spilled());
    
    // Stress test: repeatedly spill and unspill
    for i in 0..10 {
        // Remove elements to allow unspill
        cache.pop();
        cache.pop();
        assert!(cache.can_unspill());
        assert!(cache.unspill());
        assert!(!cache.is_spilled());
        
        // Add elements to trigger spill again
        cache.push(format!("x{}", i), 100 + i);
        cache.push(format!("y{}", i), 200 + i);
        cache.push(format!("z{}", i), 300 + i);
        assert!(cache.is_spilled());
        
        // Verify state consistency
        verify_dll_structure(&cache);
    }
}

#[test]
fn test_unspill_with_complex_key_types() {
    let mut cache: TinyLru<Vec<i32>, String, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill with complex keys
    cache.push(vec![1, 2], "a".to_string());
    cache.push(vec![3, 4], "b".to_string());
    cache.push(vec![5, 6], "c".to_string());
    cache.push(vec![7, 8], "d".to_string()); // Triggers spill
    assert!(cache.is_spilled());
    
    // Remove one element to allow unspill
    cache.pop();
    assert!(cache.can_unspill());
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    
    // Verify all elements are still accessible
    assert!(cache.contains_key(&vec![3, 4]));
    assert!(cache.contains_key(&vec![5, 6]));
    assert!(cache.contains_key(&vec![7, 8]));
    assert_eq!(cache.get(&vec![3, 4]), Some(&"b".to_string()));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_unspill_edge_case_empty_after_operations() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    
    // Remove all elements
    while !cache.is_empty() {
        cache.pop();
    }
    assert!(cache.is_empty());
    assert!(cache.is_spilled()); // Index persists even when empty
    
    // When empty, can_unspill() returns true because size (0) <= N (3)
    // This is technically correct - we could unspill an empty cache
    assert!(cache.can_unspill()); // Size (0) <= N (3)
    assert!(cache.unspill()); // Should succeed - unspill empty cache
    assert!(!cache.is_spilled()); // Should be back to pre-spill state
}

#[test]
fn test_unspill_preserves_index_consistency() {
    let mut cache: TinyLru<&str, i32, 3> = TinyLru::with_capacity(4);
    
    // Trigger spill
    force_spill(&mut cache);
    assert!(cache.is_spilled());
    verify_index_consistency(&cache);
    
    // Remove one element to allow unspill
    cache.pop();
    assert!(cache.can_unspill());
    
    // Unspill
    assert!(cache.unspill());
    assert!(!cache.is_spilled());
    
    // Verify all remaining elements are accessible via linear search
    assert!(cache.contains_key(&"b"));
    assert!(cache.contains_key(&"c"));
    assert!(cache.contains_key(&"d"));
    assert_eq!(cache.get(&"b"), Some(&2));
    assert_eq!(cache.get(&"c"), Some(&3));
    assert_eq!(cache.get(&"d"), Some(&4));
    
    verify_dll_structure(&cache);
}
