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
fn verify_dll_structure<K: Eq + Hash + Default, V: Default, const N: usize>(cache: &TinyLru<K, V, N>) {
    if cache.is_empty() {
        assert_eq!(cache.head, u16::MAX);
        assert_eq!(cache.tail, u16::MAX);
        return;
    }

    // Verify head and tail are valid
    assert!(cache.head < cache.size);
    assert!(cache.tail < cache.size);
    
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
    assert_eq!(count, cache.size);
}

#[test]
fn test_new() {
    let cache: TinyLru<&str, i32, 4> = TinyLru::new();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 4);
    assert!(!cache.is_spill);
    verify_dll_structure(&cache);
}

#[test]
fn test_with_capacity() {
    let cache: TinyLru<&str, i32, 4> = TinyLru::with_capacity(8);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.capacity(), 8);
    assert!(!cache.is_spill);
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
    assert!(!cache.is_spill);
    verify_dll_structure(&cache);
}

#[test]
fn test_push_multiple() {
    let mut cache = create_test_cache();
    
    assert_eq!(cache.len(), 3);
    assert!(!cache.is_empty());
    assert!(!cache.is_spill);
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
    let mut cache = create_test_cache();
    
    // Peek at existing key (should not promote)
    assert_eq!(cache.peek(&"a"), Some(&1));
    assert_eq!(cache.peek(&"b"), Some(&2));
    assert_eq!(cache.peek(&"c"), Some(&3));
    
    verify_dll_structure(&cache);
}

#[test]
fn test_peek_nonexistent() {
    let mut cache = create_test_cache();
    
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
    assert!(!cache.is_spill);
    
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
