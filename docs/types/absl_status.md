<!-- <internal link> -->

# `absl::Status` in Rust

In Google C++, the standard types for communicating an error are `absl::Status`
and `absl::StatusOr<T>`. In Rust, these are represented using `NewStatus` and
`NewStatusOr<T>` from `@abseil-cpp//absl/status` (layout-compatible with the
corresponding C++ types). For example:

```c++
absl::Status Foo();
absl::StatusOr<int> Bar();
```

This becomes:

```rust
use status::{NewStatus as Status, NewStatusOr as StatusOr};

pub fn Foo() -> Status { ... }
pub fn Bar() -> StatusOr<i32> { ... }
```

## Calling C++ APIs using `Status` {#cpp}

To enable `absl::Status` and `absl::StatusOr` bindings for C++ libraries, enable
`defines = ["CRUBIT_NEW_STATUS"]` on the `cc_library` (TODO(b/490215742): clean
this up when the old API is removed):

```python
cc_library(
    name = "cpp_api",
    srcs = ["cpp_api.cc"],
    hdrs = ["cpp_api.h"],
    aspect_hints = [
        "//features:supported",
    ],
    defines = ["CRUBIT_NEW_STATUS"],
    deps = [
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
    ],
)
```

C++ functions returning `Status`/`StatusOr` can be defined as normal:

```
{{ #include ../../examples/types/absl_status/cpp_api.h }}
```
<!--  content:ReturnsStatus -->


...and will return `NewStatus` / `NewStatusOr<T>` in Rust:

```
{{ #include ../../examples/types/absl_status/user_of_cpp_api.rs }}
```
<!--  content:ReturnsStatus -->


## Calling Rust APIs using `Status` {#rust}

Rust APIs can directly return `NewStatus` or `NewStatusOr<T>` in public
functions:

```
{{ #include ../../examples/types/absl_status/rust_api.rs }}
```
<!--  -->


`cc_bindings_from_rust` will automatically generate C++ bindings returning
`absl::Status` and `absl::StatusOr<T>`:

```
{{ #include ../../examples/types/absl_status/user_of_rust_api.cc }}
```
<!--  content:rust_api::returns_status -->


Do **not** use `StatusWrapper` or old `Result`-based `Status` / `StatusOr`
aliases.

## Working with `Status` in Rust

### Construction and Conversion

To construct status instances in Rust, use `status::ok` and `status::err`:

```rust
use status::{err, ok, NewStatus as Status, NewStatusOr as StatusOr};

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

## Migration and Future Evolution

`NewStatus` and `NewStatusOr<T>` are layout-compatible with `absl::Status` and
`absl::StatusOr<T>`, enabling zero-cost passing across the FFI boundary as well
as use in struct fields, arrays, or behind pointers and references.

Error handling with `?` is supported via the `Try` trait. Once the codebase-wide
migration is complete, `NewStatus` and `NewStatusOr` will become the default
`Status` and `StatusOr` types.
