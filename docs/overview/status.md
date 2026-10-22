# Supported Features

NOTE: The bug links below, of the form `b/123456`, are for Google-internal
tracking purposes.

This page is a high-level summary of what Crubit does and does not support
today. See the per-feature pages linked from the sidebar for details.

## Types

See [crubit.rs/types](http://crubit.rs/types) for more details about types in
general, including explanations of what it means for a type to be
ABI-compatible, layout-compatible, or bridged.

The following table summarizes the mapping of types between C++ and Rust. Rows
link to more detailed documentation where available. Most of these types are
layout-compatible; primitives and raw pointers are ABI-compatible, and
`std::string` and protocol buffers are bridged.

<!-- mdformat off(wide table) -->

| Type                                                   | C++                         | Rust                      |
| :----------------------------------------------------- | :-------------------------- | :------------------------ |
| **Primitives**                                         |                             |                           |
| Integers                                               | `int32_t`, `int64_t`, etc.  | `i32`, `i64`, etc.        |
| Floating point                                         | `float`, `double`           | `f32`, `f64`              |
| **Pointers & References**                              |                             |                           |
| Raw pointers                                           | `T*`, `const T*`            | `*mut T`, `*const T`      |
| References                                             | `const T&`, `T&`            | `&T`, `&mut T`            |
| String slices                                          | `rs_std::StrRef`            | `&str`                    |
| Slices                                                 | `rs_std::SliceRef<T>`       | `&[T]`, `&mut [T]`        |
| **Smart Pointers**                                     |                             |                           |
| [Unique pointer](http://crubit.rs/cpp/std#unique_ptr)  | `std::unique_ptr<T>`        | `cpp_std::unique_ptr<T>`  |
| [Shared pointer](http://crubit.rs/cpp/std#shared_ptr)  | `std::shared_ptr<T>`        | `cpp_std::shared_ptr<T>`  |
| **Containers**                                         |                             |                           |
| [C++ vector](http://crubit.rs/cpp/std#vector)          | `std::vector<T>`            | `cpp_std::vector<T>`      |
| Rust `Vec`                                             | `rs_std::Vec<T>`            | `Vec<T>`                  |
| Fixed-size array                                       | `std::array<T, N>`          | `[T; N]`                  |
| **Strings**                                            |                             |                           |
| [String view](http://crubit.rs/cpp/std#string_view)    | `std::string_view`          | `cpp_std::string_view`    |
| [C++ string](http://crubit.rs/cpp/std#string)          | `std::string`               | `cpp_std::string_wrapper` |
| Rust `String`                                          | `rs::alloc::string::String` | `String`                  |
| **Option & Result**                                    |                             |                           |
| [C++ optional](http://crubit.rs/cpp/std#optional)      | `std::optional<T>`          | `cpp_std::optional<T>`    |
| Rust `Option`                                          | `rs_std::Option<T>`         | `Option<T>`               |
| Rust `Result`                                          | `rs_std::Result<T, E>`      | `Result<T, E>`            |
| **Other**                                              |                             |                           |
| User-defined types                                     | `struct`, `class`           | `struct`, `enum`, `union` |
| Tuples                                                 | `rs_std::Tuple<T1, T2, ..>` | `(T1, T2, ..)`            |
| Protocol Buffers                                       | C++ proto                   | Rust proto                |
| Status                                                 | `absl::Status`              | `status::Status`          |
| StatusOr                                               | `absl::StatusOr<T>`         | `status::StatusOr<T>`     |

<!-- mdformat on -->

Not yet supported: Rust `!` (b/254507801), `()` outside of return types
(b/254099023), `std::byte` (b/213960614), and Rust `i128`/`u128` when calling
Rust from C++ (b/254094650; C++ `__int128` is supported when calling C++ from
Rust).

## Rust bindings for C++ libraries

Crubit generates Rust bindings for most everyday C++ APIs:

*   Structs, unions, and enums, including non-Rust-movable (non-trivial) types
    via the `ctor` crate.
*   Functions and methods, including overloaded constructors and assignment
    operators. Other overloads are supported when given distinct Rust names with
    [`CRUBIT_RUST_NAME`](../cpp/customizing.md).
*   `constexpr`/`const` integer, `bool`, and character-array constants (at
    namespace scope or as [static data members](../cpp/static_data_members.md)),
    which become Rust `const`s; other global variables, which become Rust
    `static`s.
*   Type aliases, forward-declared types, and concrete class template
    specializations such as `MyTemplate<int>`.

Experimental (unreleased): lifetime annotations mapped to references, and
non-virtual inheritance (upcasting, downcasting, inherited methods).

Not yet supported: automatic overload resolution (b/213280424), virtual dispatch
and overriding C++ methods from Rust (b/313733992), *safe* references,
template-generic bridging, non-type `using` aliases, constants of other types
(e.g. floating-point or class types), and macros.

## C++ bindings for Rust libraries

Crubit generates C++ bindings for most everyday Rust APIs:

*   Structs, `repr(C)` unions, and (as opaque types) enums and other unions.
*   Functions, methods, trait methods, and associated types and constants.
*   References, type aliases, `use` declarations, and simple `const`
    constants.
*   Well-known traits, mapped to their C++ equivalents: `Clone`, `Copy`,
    `Default`, `Drop`, `From`/`Into`, `PartialEq`, `PartialOrd`/`Ord`, `Hash`,
    `Display`, `Index`/`IndexMut`, `IntoIterator`, and arithmetic and bitwise
    operators. See [Traits](../rust/traits.md).
*   Defining a C++ enum from Rust.

Experimental (unreleased): non-opaque enums and non-`repr(C)` unions.

Not yet supported: defining C++ abstractions from Rust (inheriting from or
defining a C++ base class), statics and complex `const` constants, and macros.

## Usage outside of Google

Today:

*   **[Bazel](bazel.md):** `cpp_api_from_rust` and `rust_api_from_cpp` via the
    `rules_crubit` module.
*   **[CMake](cmake.md):** `cpp_api_from_rust` via Corrosion.
*   **[Cargo](cargo_build.md):** `cargo cpp_api_from_rust` builds a Cargo
    package and emits a C++ header and static library.

See [Build System Integrations](build_systems.md) for details.

Planned:

*   `rust_api_from_cpp` in CMake and Cargo builds (b/539627008).
*   Publishing `rules_crubit` to the Bazel Central Registry (b/574677342) and
    upstreaming the Corrosion changes (b/574505417).
*   Building against public Clang releases, and binary releases of Crubit.
*   Publishing the support libraries (a C++ stdlib crate, `ctor` or a
    `pin-init`-based replacement, and friends) to crates.io.
