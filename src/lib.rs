//! Simple LRU cache implementation for Rust
//!
//! Provides a generic Least Recently Used (LRU) cache with configurable capacity.
//! Automatically evicts the least recently used items when capacity is exceeded.

use std::collections::HashMap;
use std::hash::Hash;

/// Simple LRU cache implementation
pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + Hash,
{
    /// Create a new LRU cache with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Get a value by key, moving it to the most recently used position
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front
            self.order.retain(|k| k != key);
            self.order.push(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// Get a mutable reference to a value by key, moving it to the most recently used position
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.map.contains_key(key) {
            // Move to front
            self.order.retain(|k| k != key);
            self.order.push(key.clone());
            self.map.get_mut(key)
        } else {
            None
        }
    }

    /// Insert a key-value pair, evicting the least recently used item if at capacity
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing
            self.order.retain(|k| k != &key);
        } else if self.order.len() >= self.capacity {
            // Remove oldest
            if let Some(oldest) = self.order.first().cloned() {
                self.map.remove(&oldest);
                self.order.remove(0);
            }
        }

        self.order.push(key.clone());
        self.map.insert(key, value);
    }

    /// Remove a key-value pair from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k| k != key);
        self.map.remove(key)
    }

    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    /// Get the number of items in the cache
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Clear all items from the cache
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }

    /// Get the current capacity of the cache
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Resize the cache capacity, evicting items if necessary
    pub fn resize(&mut self, new_capacity: usize) {
        self.capacity = new_capacity;
        while self.order.len() > new_capacity {
            if let Some(oldest) = self.order.first().cloned() {
                self.map.remove(&oldest);
                self.order.remove(0);
            }
        }
    }
}

impl<K, V> Default for LruCache<K, V>
where
    K: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_empty() {
        let cache: LruCache<String, i32> = LruCache::new(3);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn default_cache_has_capacity_100() {
        let cache: LruCache<&str, i32> = LruCache::default();
        assert_eq!(cache.capacity(), 100);
    }

    #[test]
    fn put_and_get() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn get_mut_updates_value() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        if let Some(val) = cache.get_mut(&"a") {
            *val = 10;
        }
        assert_eq!(cache.get(&"a"), Some(&10));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache: LruCache<&str, i32> = LruCache::new(3);
        assert_eq!(cache.get(&"missing"), None);
    }

    #[test]
    fn contains_key() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        assert!(cache.contains_key(&"a"));
        assert!(!cache.contains_key(&"b"));
    }

    #[test]
    fn eviction_at_capacity() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3); // should evict "a"
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn access_refreshes_lru_order() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        // Access "a" to make it most recently used
        cache.get(&"a");
        // Insert "c" → should evict "b" (the least recently used)
        cache.put("c", 3);
        assert_eq!(cache.get(&"a"), Some(&1));
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn put_updates_existing_key() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        cache.put("a", 10);
        assert_eq!(cache.get(&"a"), Some(&10));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn remove_key() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        assert_eq!(cache.remove(&"a"), Some(1));
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn remove_missing_key() {
        let mut cache = LruCache::new(3);
        assert_eq!(cache.remove(&"a"), None);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put("a", 1);
        assert_eq!(cache.get(&"a"), Some(&1));
        cache.put("b", 2);
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn clear() {
        let mut cache = LruCache::new(3);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn resize_down() {
        let mut cache = LruCache::new(5);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);
        cache.put("d", 4);
        cache.put("e", 5);
        cache.resize(2);
        assert_eq!(cache.len(), 2);
        // Should keep the most recently used
        assert_eq!(cache.get(&"e"), Some(&5));
        assert_eq!(cache.get(&"d"), Some(&4));
    }

    #[test]
    fn resize_up() {
        let mut cache = LruCache::new(2);
        cache.put("a", 1);
        cache.put("b", 2);
        cache.resize(10);
        assert_eq!(cache.capacity(), 10);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn many_inserts() {
        let mut cache = LruCache::new(3);
        for i in 0..100 {
            cache.put(i, i * 10);
        }
        // Only the last 3 should remain
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&97), Some(&970));
        assert_eq!(cache.get(&98), Some(&980));
        assert_eq!(cache.get(&99), Some(&990));
    }

    #[test]
    fn generic_types() {
        let mut cache: LruCache<i32, String> = LruCache::new(2);
        cache.put(1, "one".to_string());
        cache.put(2, "two".to_string());
        assert_eq!(cache.get(&1), Some(&"one".to_string()));
    }
}
