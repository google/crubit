# Generating C++ `enum`s from Rust `enum`s

By default, a Rust `enum` is mapped to an opaque C++ type (see
[C++ bindings for Rust `enum`s](enums.md)). However, Crubit can try to map Rust
`enum`s to C++ `enum`s if requested using the `#[cpp_enum]` attribute. C++ code
can use such enums like any other C++ enum.

But `#[cpp_enum]` cannot be used with exhaustive Rust `enum`s. It may only be
used on non-exhaustive enums, such as those created with `#[open_enum]` from the
`open_enum` crate. Therefore, to generate C++ enum bindings, you must annotate
your Rust enum with `#[cpp_enum]`, `#[repr(...)]` (where `...` is an integer
type like `i32`), and `#[open_enum]`.

C++ enums are non-exhaustive by default, meaning they can hold values other than
the explicitly named enumerators. `#[open_enum]` generates a Rust enum that is
similarly non-exhaustive. Additionally, C++ allows multiple enumerators to have
the same value, which can be enabled in Rust by using
`#[open_enum(allow_alias)]`.

## Example

Given the following Rust crate that uses `#[cpp_enum]` and
`#[open_enum(allow_alias)]`:

```live-snippet
cs/file:examples/rust/cpp_enum/example.rs class:Color
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/rust/cpp_enum/example_generated.h class:CRUBIT_INTERNAL_RUST_TYPE|Color
```
