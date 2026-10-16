# Callbacks: `absl::AnyInvocable`

Crubit supports calling C++ functions that accept or return
[`absl::AnyInvocable`](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/any_invocable.h)
from Rust.

In Rust, `absl::AnyInvocable` parameters and return types map to standard Rust
trait objects (`Box<dyn Fn...>` and `Box<dyn FnOnce...>`).

For the reverse direction (calling Rust functions that take closures from C++),
see [C++ bindings for Rust callbacks](http://crubit.rs/rust/callbacks).

## `BUILD` configuration

To generate Rust bindings for a C++ library that uses `absl::AnyInvocable`, ensure
the `cc_library` depends on `@abseil-cpp//absl/functional:any_invocable` and is opted
into Crubit using `rust_api_from_cpp`:

```python
cc_library(
    name = "cpp_api",
    srcs = ["cpp_api.cc"],
    hdrs = ["cpp_api.h"],
    aspect_hints = [":cpp_api_rust.hint"],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

rust_api_from_cpp(
    name = "cpp_api_rust",
    cpp_target = ":cpp_api",
)

rust_test(
    name = "user_of_cpp_api",
    srcs = ["user_of_cpp_api.rs"],
    deps = [
        ":cpp_api_rust",
        "//third_party/gtest_rust/googletest",
    ],
)
```

The complete, tested version is in
[`examples/types/callbacks/BUILD`](https://github.com/google/crubit/tree/main/examples/types/callbacks/BUILD).

## Type mapping

Crubit maps `absl::AnyInvocable` to heap-allocated Rust trait objects:

C++ `absl::AnyInvocable` signature             | Rust binding type
:-------------------------------------------- | :----------------
`absl::AnyInvocable<R(Args...) const>`        | `Box<dyn Fn(Args...) -> R + Send + Sync>`
`absl::AnyInvocable<R(Args...) &&>`           | `Box<dyn FnOnce(Args...) -> R + Send>`
`absl::AnyInvocable<R(Args...)>` (non-const)  | `Box<dyn Fn(Args...) -> R + Send + Sync>`

### Non-const `absl::AnyInvocable` is promoted to `Fn`

Non-const `absl::AnyInvocable<R(Args...)>` is mapped to `Box<dyn Fn...>` (rather than
`FnMut`) in Rust.

In C++, invoking a non-const `absl::AnyInvocable` does not require exclusive
mutable access to the function object. Rust's `FnMut`, by contrast, requires an
exclusive `&mut` borrow that would be unsound if called concurrently or re-entrantly
from C++. Mapping to `Fn` guarantees that the callback is safe to invoke in any
context.

If you need to mutate state across invocations in a Rust closure passed to a non-const
`absl::AnyInvocable`, use interior mutability types such as `Atomic*` (e.g. `AtomicI32`),
`Cell`, `RefCell`, or `Mutex` instead of `&mut` capture.

## Passing callbacks from Rust to C++

C++ functions can accept `absl::AnyInvocable` by value:

```c++
// In cpp_api.h:
#include <cstdint>
#include "third_party/absl/functional/any_invocable.h"

// Accepts a callback that may be called multiple times:
int32_t ApplyMultiplier(absl::AnyInvocable<int32_t(int32_t) const> callback,
                        int32_t value);

// Accepts a single-use callback:
void InvokeOnce(absl::AnyInvocable<void() &&> callback);
```

In Rust, pass a boxed closure (`Box::new(...)`):

```rust
// In user_of_cpp_api.rs:

// 1. Calling a const callback:
let result = cpp_api::ApplyMultiplier(Box::new(|x: i32| x * 2), 21);
assert_eq!(result, 42);

// 2. Calling a single-use (FnOnce) callback:
let data = String::from("resource");
cpp_api::InvokeOnce(Box::new(move || {
    println!("Consumed: {data}");
}));
```

## Returning callbacks from C++ to Rust

C++ functions can return `absl::AnyInvocable`:

```c++
// In cpp_api.h:
absl::AnyInvocable<int32_t(int32_t) const> MakeAdder();
```

In Rust, the returned value is a callable trait object that can be called directly:

```rust
let adder = cpp_api::MakeAdder();
assert_eq!(adder(41), 42);
```

## Type aliases

Type aliases to `absl::AnyInvocable` in C++ receive corresponding type alias
bindings in Rust:

```c++
using IntMapper = absl::AnyInvocable<int32_t(int32_t) const>;
int32_t ApplyAlias(IntMapper callback, int32_t value);
```

In Rust, `cpp_api::IntMapper` is an alias for
`Box<dyn Fn(i32) -> i32 + Send + Sync>`.

## Limitations

*   **`std::function` is not bound**: Crubit does not generate bindings for
    functions taking or returning `std::function`. Replace `std::function` with
    `absl::AnyInvocable` in your C++ API, or write a thin C++ wrapper taking
    `absl::AnyInvocable`.
