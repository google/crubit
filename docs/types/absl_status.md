# `absl::Status` in Rust

In Google C++, the standard types for communicating an error are `absl::Status`
and `absl::StatusOr<T>`. In Rust, these are represented using `Status` and
`StatusOr<T>` from `@abseil-cpp//absl/status`, which are layout-compatible with
the corresponding C++ types. For example:

```c++
absl::Status Foo();
absl::StatusOr<int> Bar();
```

This becomes:

```rust
use status::{Status, StatusOr};

pub fn Foo() -> Status { ... }
pub fn Bar() -> StatusOr<i32> { ... }
```

## Calling C++ APIs using `Status` {#cpp}

Rust code that uses the status types depends on
`@abseil-cpp//absl/status:status_rust`.

C++ functions returning `Status`/`StatusOr` return the corresponding Rust types:

```live-snippet
cs/file:examples/types/absl_status/cpp_api.h content:absl::Status
```

```live-snippet
cs/file:examples/types/absl_status/user_of_cpp_api.rs content:ReturnsStatus
```

## Calling Rust APIs using `Status` {#rust}

Rust APIs can directly return `Status` or `StatusOr<T>` in public functions:

```live-snippet
cs/file:examples/types/absl_status/rust_api.rs
```

`cpp_api_from_rust` will automatically generate C++ bindings returning
`absl::Status` and `absl::StatusOr<T>`:

```live-snippet
cs/file:examples/types/absl_status/user_of_rust_api.cc content:rust_api::returns_status
```

## Working with `Status` in Rust

### Construction and Conversion

To construct status instances in Rust, `use status::{ok, err}`:

```rust
use status::{err, ok, Status, StatusOr};

let success: Status = ok(());
let value: StatusOr<i32> = ok(42);
let failure: Status = err(status::internal("error message"));
```

Existing Rust `Result` types can be converted using
`status::into_new_status(...)`.

### Testing with Googletest

When using `googletest` alongside `status::{ok, err}`, import matchers with
aliases to avoid name collisions:

```rust
use googletest::matchers::{eq, err as is_err, ok as is_ok, status_is};
use status::StatusCodeError;
```

You can then assert on `Status` or `StatusOr` values:

```rust
expect_that!(result, is_ok(eq(&42)));
expect_that!(result, is_err(status_is(StatusCodeError::INTERNAL)));
```

## Types inside `StatusOr` must be layout-compatible {#layout-compatibility}

`StatusOr<T>` is layout-compatible with C++ `absl::StatusOr<T>`, enabling
zero-cost passing across the FFI boundary. However, this layout compatibility
requires that the inner payload type `T` is **also layout-compatible** between
C++ and Rust.

Types that are bridged across the FFI boundary via runtime conversions (such as
[Protocol Buffers](protobuf.md), which are not layout-compatible between C++ and
Rust; see b/534900713, or `std::string`) do not share the same in-memory
representation between C++ and Rust. Because `T` is stored directly inside
`StatusOr<T>`, `absl::StatusOr<T>` cannot be used with these types.

As a result, functions that accept or return `absl::StatusOr<MyProto>` (or
`c9::Co<absl::StatusOr<MyProto>>`) cannot be bound directly by Crubit.

### Workaround: Out-Parameters with Proto Views

Instead of returning `absl::StatusOr<MyProto>` directly by value across FFI, the
C++ API can return `absl::Status` and accept a mutable reference (`MyProto&`)
output parameter:

```live-snippet
cs/file:examples/types/absl_status/cpp_api.h function:ReturnsStatusWithProto
```

In Rust, create an instance of the message and pass its mutable view
(`.as_mut()`):

```live-snippet
cs/file:examples/types/absl_status/user_of_cpp_api.rs function:test_returns_status_with_proto
```

## Migration and Future Evolution

`Status` and `StatusOr<T>` are layout-compatible with `absl::Status` and
`absl::StatusOr<T>`, enabling zero-cost passing across the FFI boundary as well
as use in struct fields, arrays, or behind pointers and references. Error
handling with `?` is supported via the `Try` trait.

## Deprecated API {#deprecated}

`Status` and `StatusOr` were previously named `NewStatus` and `NewStatusOr`,
which remain as aliases; prefer `Status` and `StatusOr` in new code. The legacy
`Result`-based `OldStatus` and `OldStatusOr` are deprecated, as is
`StatusWrapper`; don't use them in new code. `CRUBIT_NEW_STATUS` is no longer
needed and is ignored. Targets that have not migrated yet opt into the old
bindings with `local_defines = ["CRUBIT_OLD_STATUS"]`.
