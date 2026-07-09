# LRU Cache

A simple LRU (Least Recently Used) cache implementation for Rust with generic key-value types.

## Features

- Generic key-value types
- Configurable capacity
- Automatic eviction of least recently used items
- Mutable access to cached values
- Cache resizing support

## Usage

```rust
use lru_cache::LruCache;

let mut cache = LruCache::new(3);
cache.put("a", 1);
cache.put("b", 2);
cache.put("c", 3);

assert_eq!(cache.get(&"a"), Some(&1));
cache.put("d", 4); // evicts "b" (least recently used)
assert_eq!(cache.get(&"b"), None);
```

## License

MIT
