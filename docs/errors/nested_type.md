<!-- <internal link> -->

# Nested type errors

[Rust does not offer a way to model nested C++
types](https://github.com/rust-lang/rust/issues/8995). To compensate for this,
similar to Rust protocol buffers and their nested message support, Crubit
creates a snake_case module to hold nested types inside. For example:

```c++
struct OuterType {
  struct Inner {};
};
```

```rust
// generated Rust bindings
struct OuterType {...}
mod outer_type {
  struct Inner {...}
}
```

This will fail if the `mod outer_type` name is not unique.

### Disambiguate the names {#disambiguate}

A solution which works, but affects the C++ API in a stronger way, is to
disambiguate the names. For example, if there is an existing `namespace
outer_type`, one can rename either `OuterType` or `outer_type`. Or, if there are
two types which `snake_case` to the same name, one of them can be renamed.

Aliasing can also be used here:

```c++
struct OuterImpl {
  struct Inner {...};
};

using OuterType = OuterImpl;
```
