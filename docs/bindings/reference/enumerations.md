# Bindings for enumerations

Here we describe how Crubit maps
[enumerations](https://en.wikipedia.org/wiki/Enumerated_type): a Rust unit-only
`enum` or a C++ `enum`.

## Rust bindings for C++ `enum`s

For the following C++ header:

```live-snippet
cs/file:examples/cpp/enum/example.h class:Color
```

Crubit will generate the following bindings:

```live-snippet
cs/file:examples/cpp/enum/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

<!-- the explanation below is based on b/208944813#comment1 -->

A C++ `enum` is translated into a set of `const` items in Rust, because this
most accurately represents the fact that C++ enumerations are non-exhaustive
(i.e. in C++ any in-range value can be cast to the enumeration, even if it
wasn't listed in the `enum` declaration). In other words, C++ behavior doesn't
match Rust `enum`s where "a discriminant in an enum not included in the type
definition" is
[listed](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
as a potential source of Undefined Behavior.

TODO: Consider allowing C++ `enum`s to be marked with an attribute (e.g.
`[[crubit::exhaustive]]`?) and translate them to a Rust `enum`.

## C++ bindings for Rust unit-only `enum`s

<!-- The example below is based on the
`test_format_item_enum_with_only_discriminant_items` test from
`cc_bindings_from_rs/bindings.rs` -->

For the following Rust type:

```rust
pub enum Color {
    Red,
    Green,
    Blue,
}
```

Crubit will generate the following bindings:

```cpp
struct alignas(1) Color final {
    public:
        // The Rust type has no `Default` impl.
        Color() = delete;

        // The Rust type is not `Copy`.
        Color(const Color&) = delete;
        Color& operator=(const Color&) = delete;

        // All non-`Drop` Rust types are trivially-movable.
        Color(Color&&) = default;
        Color& operator=(Color&&) = delete;

        // The Rust type has no `Drop` impl,
        // nor requires custom drop glue.
        ~Color() = default;
    private:
        ...
};
```

Note that the generated C++ bindings are currently opaque (b/259984090 and
b/280861833 track adding more idiomatic bindings for enumerations). In
particular, the C++ side doesn't currently have any direct visibility into the
discriminant the Rust enum. Nevertheless, the bindings should cover methods and
trait of the Rust enum - for example:

*   mapping static methods from Rust to non-member methods in C++
*   mapping `Default` trait impl from Rust to the default constructor in C++
    (this bullet item is WIP - see b/258249980)
