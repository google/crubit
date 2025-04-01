# Rust bindings for C++ `enum`s

A C++ `enum` is mapped to a Rust `struct` with a similar API to a Rust `enum`.

*   The enumerated constants are present as associated constants: `MyEnum::kFoo`
    in C++ is `MyEnum::kFoo` in Rust.
*   The enum can be converted to and from its underlying type using `From` and
    `Into`. For example, `static_cast<int32_t>(x)` is `i32::from(x)` in Rust,
    and vice versa `static_cast<MyEnum>(x)` is `MyEnum::from(x)`.

However, **a C++ enum is not a Rust enum**. Some features of Rust enums are not
supported:

*   C++ enums must be converted using `From` and `Into`, not `as`.
*   C++ enums do not have exhaustive pattern matching.

## Example

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/enum/example.h class:Color
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/enum/example_generated.rs class:Color
```

## Why isn't it an `enum`?

A C++ `enum` cannot be translated directly to a Rust `enum`, because C++ enums
are "representationally non-exhaustive": a C++ `enum` can have *any* value
supported by the underlying type, even one not listed in the enumerators. For
example, in the enum above, `static_cast<Color>(42)` is a valid instance of
`Color`, even though none of `kRed`, `kBlue`, or `kGreen` have that value.

Rust enums, in contrast, are representationally exhaustive. An enum declares a
*closed* set of valid discriminants, and it is [undefined behavior][ub] to
attempt to create an enum with a value outside of that set, whether it's via
`transmute`, a raw pointer cast, or Crubit. The behavior is undefined the moment
the invalid value is created, even if it is never used.

Since a value like `static_cast<Color>(42)` is not in the list of enumerators, a
Rust `enum` cannot be used to represent an arbitrary C++ `enum`. Instead, the
Rust bindings are a `struct`. This `struct` is given the most natural and
`enum`-like API possible, though there are still gaps. (Casts using `as`, for
example, will not work with a C++ enum.)

### What about `#[non_exhaustive]`? {#non_exhaustive}

The [`#[non_exhaustive]`][ne] attribute on an enum communicates to external
crates that more variants may be added in the future, and so a `match` requires
a wildcard branch. Within the defining crate, `non_exhaustive` has no effect. It
remains undefined behavior to `transmute` from integers not declared by the
enum.

[ne]: https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
[ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
