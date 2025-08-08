<!-- <internal link> -->

# `absl::Status` in Rust

WARNING: This page documents functionality that is currently internal to the
Google monorepo.

NOTE: The APIs here have planned future backwards-incompatible changes, and you
may see LSCs as we migrate to the end state API.

In Google C++, the standard types for communicating an error is `absl::Status`.
This has support in Rust when it is directly passed by value, or returned by
value, and is mapped to a Rust `Result<(), StatusError>`. For example:

```c++
absl::Status Foo();
```

This becomes:

```rust
pub fn Foo() -> Result<(), StatusError> {...}
```

(Specifically, it will return `Status`, which is an alias for `Result<(),
StatusError>`.)

## Calling C++ APIs using `Status` {#cpp}

TODO: more narrative documentation, inline example code.

<!-- Need to submit examples first, then docs, to get working previews. -->

See examples/types/absl_status/rust_api.rs for an
example definition, and
examples/types/absl_status/user_of_rust_api.cc for
how to call it from C++.

## Calling Rust APIs using `Status` {#rust}

TODO: more narrative documentation, inline example code.

Unlike when calling C++ APIs, currently you cannot directly call a Rust API
returning a `Status`. Instead, it must use a workaround type, `StatusWrapper`.

<!-- Need to submit examples first, then docs, to get working previews. -->

See examples/types/absl_status/cpp_api.rs for an
example definition, and
examples/types/absl_status/user_of_cpp_api.rs for how
to call it from Rust.

## Future Evolution

We expect to stop using `Result`, and instead use the plain actual bindings for
`absl::Status` itself, using the `Try` trait to enable conversion into `Result`
and error handling via `?`.

This would allow `Status` to be used not only as function parameter and return
values, but also in struct fields, arrays, or behind pointers and references.

However, this is blocked on stabilization of the `Try` trait.
