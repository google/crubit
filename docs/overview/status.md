# Supported Features

NOTE: The bug links below, of the form `b/123456`, are for Google-internal
tracking purposes.

This page is a high-level summary of what Crubit does and does not support
today. See the per-feature pages linked from the sidebar for details.

## Types

The following table summarizes the mapping of types between C++ and Rust. Rows
link to more detailed documentation where available. Most of these types are
[layout-compatible](../types/index.md#abi-compatibility): they have the same
in-memory representation in both languages. Primitives and raw pointers are
additionally [ABI-compatible](../types/index.md#abi-compatibility), and
`std::string` and protocol buffers are
[bridged](../types/index.md#abi-compatibility) (converted by value). See
[Types](../types/index.md) for what each of these means in practice.

| Type                                       | C++                         | Rust                      |
| :----------------------------------------- | :-------------------------- | :------------------------ |
| **Primitives**                             |                             |                           |
| Integers                                   | `int32_t`, `int64_t`, etc.  | `i32`, `i64`, etc.        |
| Floating point                             | `float`, `double`           | `f32`, `f64`              |
| **Pointers & References**                  |                             |                           |
| Raw pointers                               | `T*`, `const T*`            | `*mut T`, `*const T`      |
| References                                 | `const T&`, `T&`            | `&T`, `&mut T`            |
| String slices                              | `rs::StrRef`                | `&str`                    |
| Slices                                     | `rs::SliceRef<T>`           | `&[T]`, `&mut [T]`        |
| **Smart Pointers**                         |                             |                           |
| [Unique pointer](../cpp/std.md#unique_ptr) | `std::unique_ptr<T>`        | `cpp_std::unique_ptr<T>`  |
| [Shared pointer](../cpp/std.md#shared_ptr) | `std::shared_ptr<T>`        | `cpp_std::shared_ptr<T>`  |
| **Containers**                             |                             |                           |
| [C++ vector](../cpp/std.md#vector)         | `std::vector<T>`            | `cpp_std::vector<T>`      |
| [Rust `Vec`](../rust/std.md#vec)           | `rs::Vec<T>`                | `Vec<T>`                  |
| Fixed-size array                           | `std::array<T, N>`          | `[T; N]`                  |
| **Strings**                                |                             |                           |
| [String view](../cpp/std.md#string_view)   | `std::string_view`          | `cpp_std::string_view`    |
| [C++ string](../cpp/std.md#string)         | `std::string`               | `cpp_std::string_wrapper` |
| [Rust `String`](../rust/std.md#string)     | `rs::alloc::string::String` | `String`                  |
| **Option & Result**                        |                             |                           |
| [C++ optional](../cpp/std.md#optional)     | `std::optional<T>`          | `cpp_std::optional<T>`    |
| [Rust `Option`](../rust/std.md#option)     | `rs::Option<T>`             | `Option<T>`               |
| [Rust `Result`](../rust/std.md#result)     | `rs::Result<T, E>`          | `Result<T, E>`            |
| **Other**                                  |                             |                           |
| User-defined types                         | `struct`, `class`           | `struct`, `enum`, `union` |
| [Tuples](../rust/std.md#tuple)             | `rs::Tuple<T1, T2, ..>`     | `(T1, T2, ..)`            |
| Protocol Buffers                           | C++ proto                   | Rust proto                |
| Status                                     | `absl::Status`              | `status::Status`          |
| StatusOr                                   | `absl::StatusOr<T>`         | `status::StatusOr<T>`     |

Not yet supported: Rust `!` (b/254507801), `()` except as a return type or a
generic argument such as `Result<(), E>` (b/254099023), `std::byte`
(b/213960614), and Rust `i128`/`u128` when calling Rust from C++ (b/254094650;
C++ `__int128` is supported when calling C++ from Rust).

## Rust bindings for C++ libraries

Crubit generates Rust bindings for everyday C++ APIs:

*   Structs, unions, and enums, including
    [non-Rust-movable](../types/non_rust_movable/intro_short.md) (non-trivial)
    types.
*   Functions and methods, including overloaded constructors and assignment
    operators. Other overloads are supported when given distinct Rust names with
    [`CRUBIT_RUST_NAME`](../cpp/customizing.md).
*   `constexpr`/`const` integer, `bool`, and character-array constants (at
    namespace scope or as [static data members](../cpp/static_data_members.md)),
    which become Rust `const`s; other global variables, which become Rust
    `static`s.
*   References, with lifetimes inferred by Rust's elision rules or given by
    [explicit lifetime annotations](../cpp/references_and_lifetimes.md).
*   Type aliases, forward-declared types, and concrete class template
    specializations such as `MyTemplate<int>`.

Unreleased: non-virtual inheritance (upcasting, downcasting, inherited methods).

Not yet supported: automatic overload resolution (b/213280424), virtual dispatch
and overriding C++ methods from Rust (b/313733992), checking that C++ code
actually honors the elided or annotated lifetimes, C++ templates with generic
(non-concrete) type parameters (see [Generics and
Templates](limits.md#generics)), non-type `using` aliases, constants of other
types (e.g. floating-point or class types), and macros.

## C++ bindings for Rust libraries

Crubit generates C++ bindings for everyday Rust APIs:

*   Structs, `repr(C)` unions, and [enums](../rust/enums.md), which become C++
    `struct`s by default or C++ `enum`s with
    [`#[cpp_enum]`](../rust/cpp_enums.md). Other user-defined types get opaque
    bindings that can be passed around but not inspected from C++.
*   Functions, methods, and trait methods, along with associated types and
    constants.
*   Type aliases, `use` declarations, and simple `const`
    constants.

> [!NOTE]
> Well-known traits are mapped to their idiomatic C++ equivalents
> rather than exposed as plain methods: `Clone`, `Copy`, `Default`, and `Drop`
> become the special member functions; `PartialEq`, `PartialOrd`/`Ord`,
> `Index`/`IndexMut`, and the arithmetic and bitwise traits become operator
> overloads; `Hash` becomes `AbslHashValue`/`std::hash`; `Display` becomes
> `operator<<`; and `IntoIterator` becomes `begin`/`end`. See
> [Traits](../rust/traits.md).

Unreleased (behind feature flags): generic types (`generics`), and richer
bindings for enums with payloads and non-`repr(C)` unions.

Not yet supported: `dyn Trait` objects, defining C++ abstractions from Rust
(inheriting from or defining a C++ base class), statics and complex `const`
constants, and macros.

## Usage outside of Google

Today:

*   **[Bazel](bazel.md):** `cpp_api_from_rust` and `rust_api_from_cpp` via the
    `rules_crubit` module.
*   **[CMake](cmake.md):** `cpp_api_from_rust` via Corrosion.
*   **[Cargo](build_systems.md#other-build-systems):** `cargo cpp_api_from_rust`
    builds a Cargo package and emits a C++ header and static library.

See [Build System Integrations](build_systems.md) for details.

Planned:

*   `rust_api_from_cpp` in CMake and Cargo builds (b/539627008).
*   Prebuilt Crubit binaries that work with public Clang and Rust releases, so
    that you don't need to build Crubit yourself against a matching toolchain.
