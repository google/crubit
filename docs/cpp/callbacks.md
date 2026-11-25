# Callbacks: `absl::AnyInvocable`, `absl::FunctionRef`, and `std::function_ref`

Crubit supports calling C++ functions that accept or return
[`absl::AnyInvocable`](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/any_invocable.h)
from Rust. It also supports `absl::FunctionRef`, `std::function_ref` (C++26),
and `rs::FnRef` for synchronous borrowed callbacks.

In Rust, `absl::AnyInvocable` parameters and return types map to standard Rust
trait objects (`Box<dyn Fn...>` and `Box<dyn FnOnce...>`). `absl::FunctionRef`,
`std::function_ref`, and `rs::FnRef` parameters map to borrowed trait objects
(`&dyn Fn...`).

For the reverse direction (calling Rust functions that take closures from C++),
see [C++ bindings for Rust callbacks](http://crubit.rs/rust/callbacks).

## `BUILD` configuration

To generate Rust bindings for a C++ library that uses `absl::AnyInvocable`,
ensure the `cc_library` depends on `@abseil-cpp//absl/functional:any_invocable`
and is opted into Crubit using `rust_api_from_cpp`:

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

| C++ `absl::AnyInvocable` signature           | Rust binding type                         |
| :------------------------------------------- | :---------------------------------------- |
| `absl::AnyInvocable<R(Args...) const>`       | `Box<dyn Fn(Args...) -> R + Send + Sync>` |
| `absl::AnyInvocable<R(Args...) &&>`          | `Box<dyn FnOnce(Args...) -> R + Send>`    |
| `absl::AnyInvocable<R(Args...)>` (non-const) | `Box<dyn Fn(Args...) -> R + Send + Sync>` |

### Non-const `absl::AnyInvocable` is promoted to `Fn`

Non-const `absl::AnyInvocable<R(Args...)>` is mapped to `Box<dyn Fn...>` (rather
than `FnMut`) in Rust.

In C++, invoking a non-const `absl::AnyInvocable` does not require exclusive
mutable access to the function object. Rust's `FnMut`, by contrast, requires an
exclusive `&mut` borrow that would be unsound if called concurrently or
re-entrantly from C++. Mapping to `Fn` guarantees that the callback is safe to
invoke in any context.

If you need to mutate state across invocations in a Rust closure passed to a
non-const `absl::AnyInvocable`, use interior mutability types such as `Atomic*`
(e.g. `AtomicI32`), `Cell`, `RefCell`, or `Mutex` instead of `&mut` capture.

### Borrowed Callbacks (`absl::FunctionRef` / `std::function_ref` / `rs::FnRef`)

In C++, synchronous callbacks that do not take ownership of state are passed by
value using `absl::FunctionRef`, `std::function_ref` (C++26), or `rs::FnRef`.
Crubit binds these parameters to borrowed Rust trait objects:

| C++ parameter type                                                      | Rust binding type                        |
| :---------------------------------------------------------------------- | :--------------------------------------- |
| `absl::FunctionRef<R(Args...)>` / `absl::FunctionRef<R(Args...) const>` | `&'a (dyn Fn(Args...) -> R + Sync + 'a)` |
| `std::function_ref<R(Args...)>` / `std::function_ref<R(Args...) const>` | `&'a (dyn Fn(Args...) -> R + Sync + 'a)` |
| `rs::FnRef<R(Args...)>` / `rs::FnRef<R(Args...) const>`                 | `&'a (dyn Fn(Args...) -> R + 'a)`        |

(`noexcept` function signatures map to the same Rust types.)

```cpp
void Sort(absl::FunctionRef<bool(int, int)> compare);
```

translates to Rust as:

```rust
pub fn Sort(compare: &(dyn Fn(i32, i32) -> bool + Sync + '_));
```

Because C++ does not enforce exclusive `&mut` access when invoking a callback,
both `R(Args...)` and `R(Args...) const` signatures map to `Fn` rather than
`FnMut`.

#### Threading contract (`Sync` vs. non-`Sync`)

*   **`absl::FunctionRef` and `std::function_ref` require `Sync`**: C++ has no
    `Send`/`Sync` distinction, and C++ APIs accepting `absl::FunctionRef` or
    `std::function_ref` (such as parallel-for helpers) may invoke the callback
    concurrently from worker threads during the call. `Send` is not required
    because the closure is only shared by reference, never moved (`&F: Send`
    holds whenever `F: Sync`). Use thread-safe interior mutability (`Atomic*`,
    `Mutex`) if the closure mutates captured state.
*   **`rs::FnRef` does not require `Sync`**: Crubit defines `rs::FnRef` as the
    C++ counterpart to Rust's `&dyn Fn`, which may only be invoked on the
    calling thread. Omitting `Sync` allows Rust callers to pass closures that
    capture `!Sync` types such as `Cell`, `RefCell`, or `Rc`. C++ callees
    receiving `rs::FnRef` must never invoke it from another thread.

#### Lifetime and non-retention contract

The C++ callee **must not retain or copy a borrowed callback past the return of
the call** (for example, by storing it in a global, heap object, or `this`). As
[Abseil's `absl::FunctionRef`
documentation](https://github.com/abseil/abseil-cpp/blob/master/absl/functional/function_ref.h)
states, `absl::FunctionRef` is suitable only as a function parameter and must
not be used as a return value or struct field. Crubit enforces the return-value
half of this contract at binding-generation time by rejecting any function or
method whose return type is, or transitively contains (through struct fields,
base classes, or bridge template arguments), a borrowed callable.

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

In Rust, the returned value is a callable trait object that can be called
directly:

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
