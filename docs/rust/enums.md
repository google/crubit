# C++ bindings for Rust `enum`s

A Rust `enum` is mapped to an opaque C++ type. C++ code cannot create a specific
variant, but can call functions accepting or returning an `enum`.

To receive C++ bindings, the `enum` must be movable in C++. See
[Movable Types](movable_types).

## Example

Given the following Rust crate:

```live-snippet
cs/file:examples/rust/enum/example.rs class:Color
```

Crubit will generate the following bindings:

<!-- Note: Kythe currently indexes this as class `CRUBIT_INTERNAL_RUST_TYPE` because it doesn't have a build rule. -->

```live-snippet
cs/file:examples/rust/enum/example_generated.h class:CRUBIT_INTERNAL_RUST_TYPE|Color
```

## Why isn't it a C++ `enum`? {#cpp_enum}

A `repr(i32)` or fieldless `repr(C)` `enum` is very similar to a C++ `enum`.
However, Rust enums are exhaustive: any value not explicitly listed in the
`enum` declaration does not exist, and it is
[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
to attempt to create one.

C++ `enum`s, in contrast, are "non-exhaustive": a C++ `enum` can have *any*
value supported by the underlying type, even one not listed in the enumerators.
For example, if the above example were a C++ enum, `static_cast<Color>(42)`
would be a valid instance of `Color`, even though neither `Red`, `Blue`, nor
`Green` have that value.

In order to prevent invalid Rust values from being produced by C++, a C++ `enum`
cannot be used to represent a Rust `enum`. Instead, the C++ bindings are a
`struct`, even for fieldless `enum`s.

## C++ movable {#cpp_movable}

To receive C++ bindings, the `enum` must be movable in C++. See
[Movable Types](movable_types).
