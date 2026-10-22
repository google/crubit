# Callbacks: `absl::AnyInvocable` and `Fn`/`FnMut`/`FnOnce`

Crubit supports bidirectional function callbacks across C++ and Rust:

*   **[Rust bindings for C++ callbacks
    (`absl::AnyInvocable`)](../cpp/callbacks.md)**: Calling C++ APIs that take
    or return `absl::AnyInvocable` from Rust.
*   **[C++ bindings for Rust callbacks (`Fn`, `FnMut`,
    `FnOnce`)](../rust/callbacks.md)**: Calling Rust APIs that accept closures
    from C++ via `rs::FnRef` and `rs::Fn`.

## Type mappings

### C++ callbacks in Rust

<!-- mdformat off(wide table) -->

| C++ type                                  | Rust type                             | Semantics             |
| :---------------------------------------- | :------------------------------------ | :-------------------- |
| `absl::AnyInvocable<R(...) const>`        | `Box<dyn Fn(...) -> R + Send + Sync>`     | Multiple invocations  |
| `absl::AnyInvocable<R(...) &&>`           | `Box<dyn FnOnce(...) -> R + Send + Sync>` | Single-use invocation |
| `absl::AnyInvocable<R(...)>` (non-const)  | `Box<dyn Fn(...) -> R + Send + Sync>`     | Promoted to `Fn`      |

<!-- mdformat on -->

### Rust callbacks in C++

<!-- mdformat off(wide table) -->

| C++ type                     | Rust type                                            | Semantics                                        |
| :--------------------------- | :--------------------------------------------------- | :----------------------------------------------- |
| `rs::FnRef<R(...) const>`    | `&dyn Fn(...) -> R` / `impl Fn(...) -> R`            | Borrowed, zero allocation                        |
| `rs::FnRef<R(...)>`          | `&mut dyn FnMut(...) -> R` / `impl FnMut(...) -> R`  | Borrowed mutable, zero allocation                |
| `rs::Fn<R(...) const>`       | `Box<dyn Fn(...) -> R>`                              | Owned                                            |
| `rs::Fn<R(...)>`             | `Box<dyn FnMut(...) -> R>`                           | Owned mutable                                    |
| `rs::Fn<R(...) &&>`          | `Box<dyn FnOnce(...) -> R>` / `impl FnOnce(...) + 'static` | Owned, single-use (`std::move`)            |
| `rs_std::DynCallable<Sig>`   | (Rust callback in struct)                            | Implicitly converts to `absl::AnyInvocable<Sig>` |

<!-- mdformat on -->

## Examples

Runnable, tested examples for both directions are in
[`examples/types/callbacks/`](https://github.com/google/crubit/tree/main/examples/types/callbacks/).
