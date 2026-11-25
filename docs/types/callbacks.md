# Callbacks: `absl::AnyInvocable` and `Fn`/`FnMut`/`FnOnce`

Crubit supports bidirectional function callbacks across C++ and Rust:

*   **[Rust bindings for C++ callbacks (`absl::AnyInvocable`,
    `absl::FunctionRef`, `std::function_ref`)](../cpp/callbacks.md)**: Calling
    C++ APIs that take or return `absl::AnyInvocable`, or take borrowed
    callbacks (`absl::FunctionRef`, `std::function_ref`, `rs::FnRef`), from
    Rust.
*   **[C++ bindings for Rust callbacks (`Fn`, `FnMut`,
    `FnOnce`)](../rust/callbacks.md)**: Calling Rust APIs that accept closures
    from C++ via `rs::FnRef` and `rs::Fn`.

## Type mappings

### C++ callbacks in Rust

| C++ type                                     | Rust type                             | Semantics                                |
| :------------------------------------------- | :------------------------------------ | :--------------------------------------- |
| `absl::AnyInvocable<R(...) const>`           | `Box<dyn Fn(...) -> R + Send + Sync>` | Owned, multiple invocations              |
| `absl::AnyInvocable<R(...) &&>`              | `Box<dyn FnOnce(...) -> R + Send>`    | Owned, single-use invocation             |
| `absl::AnyInvocable<R(...)>` (non-const)     | `Box<dyn Fn(...) -> R + Send + Sync>` | Owned, promoted to `Fn`                  |
| `absl::FunctionRef<R(...)>` / `R(...) const` | `&'a (dyn Fn(...) -> R + Sync + 'a)`  | Borrowed parameter, thread-safe (`Sync`) |
| `std::function_ref<R(...)>` / `R(...) const` | `&'a (dyn Fn(...) -> R + Sync + 'a)`  | Borrowed parameter (C++26), `Sync`       |
| `rs::FnRef<R(...)>` / `R(...) const`         | `&'a (dyn Fn(...) -> R + 'a)`         | Borrowed parameter, calling thread only  |

### Rust callbacks in C++

| C++ type                  | Rust type                                                  | Semantics                                        |
| :------------------------ | :--------------------------------------------------------- | :----------------------------------------------- |
| `rs::FnRef<R(...) const>` | `&dyn Fn(...) -> R` / `impl Fn(...) -> R`                  | Borrowed, zero allocation, calling thread only   |
| `rs::FnRef<R(...)>`       | `&mut dyn FnMut(...) -> R` / `impl FnMut(...) -> R`        | Borrowed mutable, zero allocation                |
| `rs::Fn<R(...) const>`    | `Box<dyn Fn(...) -> R>`                                    | Owned                                            |
| `rs::Fn<R(...)>`          | `Box<dyn FnMut(...) -> R>`                                 | Owned mutable                                    |
| `rs::Fn<R(...) &&>`       | `Box<dyn FnOnce(...) -> R>` / `impl FnOnce(...) + 'static` | Owned, single-use (`std::move`)                  |
| `rs::DynCallable<Sig>`    | (Rust callback in struct)                                  | Implicitly converts to `absl::AnyInvocable<Sig>` |

## Examples

Runnable, tested examples for both directions are in
[`examples/types/callbacks/`](https://github.com/google/crubit/tree/main/examples/types/callbacks/).
