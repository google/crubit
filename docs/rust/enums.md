# C++ bindings for Rust `enum`s

A Rust `enum` is mapped to an opaque C++ type.

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

An `enum` does not need to be movable in C++ to receive bindings. Whether the
generated C++ class has move and copy operations depends on which traits the
Rust type implements: see [Movable Types](movable_types.md).

If the `enum` is not C++-movable (for example, it implements `Drop` but neither
`Default` nor `Clone`), its C++ move constructor and move assignment operator
are deleted. It can still be used by reference and returned by value, and
functions that take it by value accept
[`rs::Movable<T>`](movable_types.md#non_cpp_movable) instead of `T`.

## Enums with payload

Each variant of a Rust `enum` may contain an additional payload (a tuple or a
struct). C++ bindings for Rust `enum`s provide the following ways of working
with an `enum` payload:

*   Constructing an `enum` variant with the given payload by calling a `static`
    `Make<variant name>` method (one such method is injected for each of `enum`
    variants). The following bugs track future work in this area:

    *   b/487357254: Constructing variants with a struct payload

*   TODO(b/262737383): Matching `enum` variants and inspecting their payload.

### Example

Given the following Rust crate:

<!-- b/487357254: Cover struct payload in this example. -->

```live-snippet
cs/file:examples/rust/enum_with_payload/example.rs class:Color
```

Crubit will generate the following bindings:

<!-- Note: Kythe currently indexes this as class `CRUBIT_INTERNAL_RUST_TYPE` because it doesn't have a build rule. -->

```live-snippet
cs/file:examples/rust/enum_with_payload/example_generated.h class:CRUBIT_INTERNAL_RUST_TYPE|Color
```
