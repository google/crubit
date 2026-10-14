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

```
{{ #include ../../examples/types/absl_status/cpp_api.h }}
```
<!--  function:ReturnsStatus\b|ReturnsStatusOrInt\b -->


```
{{ #include ../../examples/types/absl_status/user_of_cpp_api.rs }}
```
<!--  function:test_returns_status\b|test_returns_status_or_int\b -->


## Calling Rust APIs using `Status` {#rust}

Rust APIs can directly return `Status` or `StatusOr<T>` in public functions:

```
{{ #include ../../examples/types/absl_status/rust_api.rs }}
```
<!--  -->


`cpp_api_from_rust` will automatically generate C++ bindings returning
`absl::Status` and `absl::StatusOr<T>`:

```
{{ #include ../../examples/types/absl_status/user_of_rust_api.cc }}
```
<!--  content:rust_api::returns_status -->


Passing `StatusOr<T>` by value to or from C++ requires `T` to be
[C++-movable](/docs/rust/movable_types.md) (no `Drop`,
`Clone`, or `Drop + Default`; supporting non-C++-movable Rust types via
`derive_rust_movable` is tracked in b/545695274). Otherwise, derive `Default` or
`Clone` on `T`, wrap `T` in `Box<T>` or `Vec<T>`, or return `Status` with an
`&mut T` out-parameter.

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
use googletest::matchers::{err as is_err, ok as is_ok, status_is};
```

You can then assert on `Status` or `StatusOr` values:

```rust
expect_that!(result, is_ok(eq(&42)));
expect_that!(result, is_err(status_is(StatusCode::Internal)));
```

## Non-Rust-movable C++ types and `std::string` {#non-rust-movable}

When `T` is a
[non-Rust-movable](/docs/cpp/classes_and_structs.md#rust_movable)
(`!Unpin`) C++ type, `StatusOr<T>` is also `!Unpin`: C++ functions returning
`absl::StatusOr<T>` return a `Ctor` (initialized with `ctor::emplace!`; borrow
via `(&*res).as_ref()?` / `res.as_ok_ref()` or mutate via
`res.as_mut().as_ok_pin_mut()`).

For `absl::StatusOr<std::string>`, default `std::string`
(`cpp_std::string_wrapper`, b/408961701) is not layout-compatible (see
[below](#layout-compatibility)). Adding
`//features:layout_compat_string` to `aspect_hints` maps
`std::string` to the layout-compatible `cpp_std::string` (`!Unpin` due to SSO
self-references, so `derive_rust_movable` does not apply, but C++-movable via
`cpp_move_constructible`), which works from Rust via `ctor::emplace!` (or use an
out-parameter, `absl::string_view`, or `std::unique_ptr` / `Box` when
`layout_compat_string` is not enabled or when returning from Rust to C++):

```
{{ #include ../../examples/types/absl_status/cpp_api.h }}
```
<!--  function:ReturnsStatusOrString -->


```
{{ #include ../../examples/types/absl_status/user_of_cpp_api.rs }}
```
<!--  function:test_returns_status_or_string -->


## Types inside `StatusOr` must be layout-compatible {#layout-compatibility}

`StatusOr<T>` is layout-compatible with C++ `absl::StatusOr<T>`, enabling
zero-cost passing across the FFI boundary. However, this layout compatibility
requires that the inner payload type `T` is **also layout-compatible** between
C++ and Rust.

Types that are bridged across the FFI boundary via runtime conversions (such as
[Protocol Buffers](/docs/types/protobuf.md), which are not
layout-compatible between C++ and Rust; see b/534900713, or `std::string`
without `layout_compat_string`) do not share the same in-memory representation
between C++ and Rust. Because `T` is stored directly inside `StatusOr<T>`,
`absl::StatusOr<T>` cannot be used with these types.

As a result, functions that accept or return `absl::StatusOr<MyProto>` (or
`c9::Co<absl::StatusOr<MyProto>>`) cannot be bound directly by Crubit.

### Workaround: Out-Parameters with Proto Views

Instead of returning `absl::StatusOr<MyProto>` directly by value across FFI, the
C++ API can return `absl::Status` and accept a mutable reference (`MyProto&`)
output parameter:

```
{{ #include ../../examples/types/absl_status/cpp_api.h }}
```
<!--  function:ReturnsStatusWithProto -->


In Rust, create an instance of the message and pass its mutable view
(`.as_mut()`):

```
{{ #include ../../examples/types/absl_status/user_of_cpp_api.rs }}
```
<!--  function:test_returns_status_with_proto -->


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
