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

---

## Part of Orkid Labs

**lru-cache** — Simple, dependency-free LRU cache for Rust with generic keys and values.

- Install: `cargo add orkid-lru-cache`
- Package: https://crates.io/crates/orkid-lru-cache
- API docs: https://docs.rs/orkid-lru-cache
- Docs: https://docs.orkidlabs.xyz/packages/lru-cache

Related Orkid projects: [event-broadcaster](https://github.com/orkid-labs/event-broadcaster)

Orkid Labs builds gasless, MEV-protected swap routing and agent-native DeFi infrastructure — [orkidlabs.xyz](https://orkidlabs.xyz) · [@orkid-labs on npm](https://www.npmjs.com/org/orkid-labs) · [github.com/orkid-labs](https://github.com/orkid-labs)
