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

If `mod outer_type` is already in use (for example, because `OuterType` is
already `snake_case`, because `namespace outer_type` exists, or because another
type in the same scope `snake_case`s to `outer_type`), Crubit disambiguates the
module name by appending `_items` (and additional trailing `_` characters if
needed, such as `mod outer_type_items`).

### Disambiguate the names {#disambiguate}

To avoid the disambiguated `_items` suffix (or if an older version of Crubit
reports a module name collision), you can disambiguate the C++ names directly.
For example, if there is an existing `namespace outer_type`, one can rename
either `OuterType` or `outer_type`. Or, if there are two types which
`snake_case` to the same name, one of them can be renamed.

Aliasing can also be used here:

```c++
struct OuterImpl {
  struct Inner {...};
};

using OuterType = OuterImpl;
```
