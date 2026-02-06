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

This will fail if the `mod outer_type` name is not unique, and due to
b/481667188: if the `Inner` type is a forward declaration, which is completed
outside of the `OuterType` type. If that happens, `Inner` will not receive
bindings.

## Fix

### Type Alias {#alias}

The most general fix, which will ensure `Inner` gets bindings if it can, is to
move it outside of `OuterType`:

```c++
struct Inner {};
struct OuterType {
  using Inner = Inner;
};
```

Even if the bindings for the `using Inner = Inner;` fail for any of the above
reasons, the top-level `struct Inner` will still receive bindings, if it can.

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

### b/481667188: Complete the type

TODO(b/481667188): Completing the type inside the class should not be necessary.

For now, avoid this pattern if `Inner` is meant to be used from Rust:

```c++
struct OuterType {
  struct Inner;
};
struct OuterType::Inner {...};
```

Defining the complete type inside of `OuterType` will work, as will the general
[type alias solution](#alias) above.
