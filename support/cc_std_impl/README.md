This directory contains extra Rust customizations for `cc_std`.

## Making backwards-incompatible changes

Like most directories in `support/`, changes to this file take effect instantly.
To make version-dependent changes, pair them with a feature flag and cfg guard.

For example, consider std::vector: a new pending release might change the layout
of `std::vector`, so that instead of using begin/end/capacity_end pointers, it
uses a begin pointer and size/capacity integers. This can be released as so:

```rust
// cc_std_impl/unique_ptr.rs
#[cfg(not(feature = "len_capacity_encoding"))]
#[repr(C)]
pub struct vector<T> {
    begin: *const T,
    _end: *const T,
    _capacity_end: *const T,
}

/// 2. This layout is experimental.
#[cfg(feature = "len_capacity_encoding")]
#[repr(C)]
pub struct vector<T> {
    begin: *const T,
    len: usize,
    capacity: usize,
}
```

```python
# cc_std/BUILD
bindings_for_toolchain_headers(
    name = "cc_std",
    ...
    crate_features = ["len_capacity_encoding"],
    ...
)
```

The old release, which contains the *old* copy of `BUILD`, will not pass the
`len_capacity_encoding` feature, and will use the previous layout.

Crubit when built at head, and for the next release, will use the *new* copy of
BUILD, which sets the feature, and obtains the new struct definition.

Even though the source code is live-at-head, changes like this will be
release-gated through the crate features, which are not.
