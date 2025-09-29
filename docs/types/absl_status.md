<!-- <internal link> -->

# `absl::Status` in Rust

NOTE: The APIs here have planned future backwards-incompatible changes, and you
may see LSCs as we migrate to the end state API.

In Google C++, the standard types for communicating an error are `absl::Status`
and `absl::StatusOr<T>`. These have support in Rust when they are directly
passed by value, or returned by value, and are mapped to a Rust `Result`. For
example:

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

C++ functions returning `Status`/`StatusOr` can be defined as normal:

```live-snippet
cs/file:examples/types/absl_status/cpp_api.h content:ReturnsStatus
```

...and will return a `Result`:

```live-snippet
cs/file:examples/types/absl_status/user_of_cpp_api.rs content:ReturnsStatus
```

## Calling Rust APIs using `Status` {#rust}

Unlike when calling C++ APIs, currently you cannot directly call a Rust API
returning a `Status` or `StatusOr`. Instead, it must use a workaround type,
`StatusWrapper`. This is tracked by b/441266536.

```live-snippet
cs/file:examples/types/absl_status/rust_api.rs
```

The `StatusWrapper` type automatically becomes an `absl::Status` in C++:

```live-snippet
cs/file:examples/types/absl_status/user_of_rust_api.cc content:rust_api::ReturnsStatus
```

## Future Evolution

We expect to stop using `Result`, and instead use the plain actual bindings for
`absl::Status` itself, using the `Try` trait to enable conversion into `Result`
and error handling via `?`.

This would allow `Status` to be used not only as function parameter and return
values, but also in struct fields, arrays, or behind pointers and references.

However, this is blocked on stabilization of the `Try` trait.
