---
name: lru-cache
description: Simple, dependency-free LRU cache for Rust with generic keys and values.
---

# lru-cache

A minimal least-recently-used cache: O(1) get/put, generic K/V, no async runtime or unsafe required. Extracted from Orkid's quote hot paths where allocation-free lookups matter.

## When to use

You need an in-process LRU cache without pulling in tokio or a full caching framework.

## Install

```
cargo add orkid-lru-cache
```

## Links

- Repository: https://github.com/orkid-labs/lru-cache
- Package: https://crates.io/crates/orkid-lru-cache
- Documentation: https://docs.rs/orkid-lru-cache
- Orkid Labs: https://orkidlabs.xyz
