# C++ bindings for Rust type aliases.

For the following Rust type:

```rust
pub type TypeAlias = i32;
```

Crubit will generate the following bindings:

```cpp
using TypeAlias = std::int32_t;
```
