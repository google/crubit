# Callbacks: `Fn`, `FnMut`, `FnOnce`

Crubit supports exporting Rust functions that take closures (`Fn`, `FnMut`,
`FnOnce`) to C++ via `cpp_api_from_rust`.

In C++, callable parameters are represented using `rs::FnRef` (for borrowed
callbacks) and `rs::Fn` (for owned callbacks). C++ callers can satisfy these
parameters using C++ lambdas, function pointers, `std::function`, or
`absl::AnyInvocable`.

For the reverse direction (calling C++ APIs taking `absl::AnyInvocable` from Rust),
see [Rust bindings for C++ callbacks](http://crubit.rs/cpp/callbacks).

## `BUILD` configuration

When generating C++ bindings for a Rust library via `cpp_api_from_rust`:

```python
rust_library(
    name = "rust_api",
    srcs = ["rust_api.rs"],
)

cpp_api_from_rust(
    name = "rust_api_cc",
    crate = ":rust_api",
)

cc_test(
    name = "user_of_rust_api",
    srcs = ["user_of_rust_api.cc"],
    deps = [
        ":rust_api_cc",
        "//testing/base/public:gunit_main",
    ],
)
```

The complete, tested version is in
[`examples/types/callbacks/BUILD`](https://github.com/google/crubit/tree/main/examples/types/callbacks/BUILD).

## Type mapping

Rust functions can take callable parameters as borrowed references, owned boxes,
or generic `impl Trait` parameters:

Rust parameter type                               | Generated C++ parameter type      | Ownership / Allocation
:------------------------------------------------ | :-------------------------------- | :---------------------
`&dyn Fn(Args...) -> R`                           | `rs::FnRef<R(Args...) const>`     | Borrowed (no allocation)
`&mut dyn FnMut(Args...) -> R`                    | `rs::FnRef<R(Args...)>`           | Borrowed (no allocation)
`impl Fn(Args...) -> R`                           | `rs::FnRef<R(Args...) const>`     | Borrowed (no allocation)
`impl FnMut(Args...) -> R`                        | `rs::FnRef<R(Args...)>`           | Borrowed (no allocation)
`impl FnOnce(Args...) -> R` (without `'static`)   | `rs::FnRef<R(Args...)>`           | Borrowed (no allocation)
`Box<dyn Fn(Args...) -> R>`                       | `rs::Fn<R(Args...) const>`        | Owning (heap-allocated)
`Box<dyn FnMut(Args...) -> R>`                    | `rs::Fn<R(Args...)>`              | Owning (heap-allocated)
`Box<dyn FnOnce(Args...) -> R>`                   | `rs::Fn<R(Args...) &&>`           | Owning (heap-allocated)
`impl FnOnce(Args...) -> R + 'static`             | `rs::Fn<R(Args...) &&>`           | Owning (heap-allocated)

The generated headers include the necessary support headers automatically:
*   `rs::FnRef`: `#include "support/rs_std/fn_ref.h"`
*   `rs::Fn`: `#include "support/rs_std/fn.h"`

## Passing callbacks from C++ to Rust

Given a Rust library defining functions that accept closures:

```rust
// In rust_api.rs:

// Borrowed reference: zero allocation
pub fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// Mutable borrowed reference: zero allocation
pub fn apply_fn_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    f(x)
}

// Owned callback: invoked once
pub fn apply_fn_once(f: Box<dyn FnOnce(i32) -> i32>, x: i32) -> i32 {
    f(x)
}
```

C++ callers can pass C++ lambdas, function pointers, `std::function`, or
`absl::AnyInvocable`:

```c++
#include "examples/types/callbacks/rust_api.h"

// 1. Pass a C++ lambda to an `impl Fn` parameter:
int32_t res1 = rust_api::apply_fn([](int32_t x) { return x * 2; }, 21);
// res1 == 42

// 2. Pass an absl::AnyInvocable:
absl::AnyInvocable<int32_t(int32_t) const> cb = [](int32_t x) { return x * 3; };
int32_t res2 = rust_api::apply_fn(cb, 14);
// res2 == 42

// 3. Pass a mutable lambda to `impl FnMut`:
int32_t total = 0;
auto mutable_lambda = [&total](int32_t x) mutable {
  total += x;
  return total;
};
rust_api::apply_fn_mut(mutable_lambda, 10);
rust_api::apply_fn_mut(mutable_lambda, 32);
// total == 42

// 4. Pass a move-only lambda to `Box<dyn FnOnce>`:
auto ptr = std::make_unique<int32_t>(10);
int32_t res3 = rust_api::apply_fn_once(
    [p = std::move(ptr)](int32_t x) { return x + *p; }, 32);
// res3 == 42
```

## `rs::FnRef` vs. `rs::Fn`

*   **`rs::FnRef` (borrowed)**: Non-owning view of a callable, similar to
    `absl::FunctionRef`. It stores a pointer to the callable and an invoker
    trampoline. It performs **no heap allocation** and must not outlive the
    function call.
*   **`rs::Fn` (owned)**: Owning callable wrapper. It heap-allocates the
    provided callable and manages its destruction, making it suitable for
    storing in Rust structs or transferring ownership across threads.
    `rs::Fn<Sig &&>` indicates a callable that can only be invoked once; call it
    using `std::move(f)(...)`.

## Converting Rust callbacks to `absl::AnyInvocable` (`rs_std::DynCallable`)

When C++ code receives an owned Rust callback (for example, stored in a Rust struct
field exposed to C++), the C++ type is `rs_std::DynCallable<Sig>`
(`support/rs_std/dyn_callable.h`).

`rs_std::DynCallable` is a move-only functional wrapper around Rust's `Box<dyn Trait>`.
It is **implicitly convertible** to `absl::AnyInvocable`:

```c++
rs_std::DynCallable<void() &&> rust_callback = ...;
absl::AnyInvocable<void() &&> any_invocable = std::move(rust_callback);
```

Because `rs_std::DynCallable` uses an internal representation compatible with
`absl::AnyInvocable`, this conversion is efficient and avoids re-wrapping or additional
heap allocations.
