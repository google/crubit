# Callbacks: `absl::AnyInvocable` and `Fn`/`FnMut`/`FnOnce`

Crubit supports bidirectional function callbacks across C++ and Rust:

*   **[Rust bindings for C++ callbacks
    (`absl::AnyInvocable`)](../cpp/callbacks.md)**: Calling C++ APIs that take
    or return `absl::AnyInvocable` from Rust.
*   **[C++ bindings for Rust callbacks (`Fn`, `FnMut`,
    `FnOnce`)](../rust/callbacks.md)**: Calling Rust APIs that accept closures
    from C++ via `rs::FnRef` and `rs::Fn`.

## Type mappings

| Direction       | C++ type                                 | Rust type                                                  | Semantics                                        |
| :-------------- | :--------------------------------------- | :--------------------------------------------------------- | :----------------------------------------------- |
| C++ $\\to$ Rust | `absl::AnyInvocable<R(...) const>`       | `Box<dyn Fn(...) -> R + Send + Sync>`                      | Multiple invocations                             |
| C++ $\\to$ Rust | `absl::AnyInvocable<R(...) &&>`          | `Box<dyn FnOnce(...) -> R + Send + Sync>`                  | Single-use invocation                            |
| C++ $\\to$ Rust | `absl::AnyInvocable<R(...)>` (non-const) | `Box<dyn Fn(...) -> R + Send + Sync>`                      | Promoted to `Fn`                                 |
| Rust $\\to$ C++ | `rs::FnRef<R(...) const>`                | `&dyn Fn(...) -> R` / `impl Fn(...) -> R`                  | Borrowed, zero allocation                        |
| Rust $\\to$ C++ | `rs::FnRef<R(...)>`                      | `&mut dyn FnMut(...) -> R` / `impl FnMut(...) -> R`        | Borrowed mutable, zero allocation                |
| Rust $\\to$ C++ | `rs::Fn<R(...) const>`                   | `Box<dyn Fn(...) -> R>`                                    | Owned                                            |
| Rust $\\to$ C++ | `rs::Fn<R(...)>`                         | `Box<dyn FnMut(...) -> R>`                                 | Owned mutable                                    |
| Rust $\\to$ C++ | `rs::Fn<R(...) &&>`                      | `Box<dyn FnOnce(...) -> R>` / `impl FnOnce(...) + 'static` | Owned, single-use (`std::move`)                  |
| Rust $\\to$ C++ | `rs::DynCallable<Sig>`                   | (Rust callback in struct)                                  | Implicitly converts to `absl::AnyInvocable<Sig>` |

## Examples

Runnable, tested examples for both directions are in
[`examples/types/callbacks/`](https://github.com/google/crubit/tree/main/examples/types/callbacks/).
