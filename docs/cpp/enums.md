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
cs/file:examples/cpp/enum/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

## Why isn't it an `enum`?

A C++ `enum` cannot be translated directly to a Rust `enum`, because C++ enums
are "non-exhaustive": a C++ `enum` can have *any* value supported by the
underlying type, even one not listed in the enumerators. For example, in the
enum above, `static_cast<Color>(42)` is a valid instance of `Color`, even though
none of `kRed`, `kBlue`, or `kGreen` have that value.

Rust enums, in contrast, are exhaustive: any value not explicitly listed in the
`enum` declaration does not exist, and it is
[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
to attempt to create one.

Since a value like `static_cast<Color>(42)` is not in the list of enumerators, a
Rust `enum` cannot be used to represent an arbitrary C++ `enum`. Instead, the
Rust bindings are a `struct`. This `struct` is given the most natural and
`enum`-like API possible, though there are still gaps. (Casts using `as`, for
example, will not work with a C++ enum.)
