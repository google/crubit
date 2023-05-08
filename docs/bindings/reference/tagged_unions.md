# Bindings for tagged unions

Here we describe how Crubit maps
[tagged unions](https://en.wikipedia.org/wiki/Tagged_union): non-unit-only
`enum` in Rust, and `std::variant` in C++. (Rust unit-only enums are treated
like C++ enums, and handled [separately](./enumerations.md).)

## Rust bindings for C++ `std::variant`

Crubit has no special knowledge about
[`std::variant`](https://en.cppreference.com/w/cpp/utility/variant) - it will be
handled as any other class template.

## C++ bindings for Rust `enum`

<!-- The example below is based on the
`test_format_item_enum_with_tuple_and_struct_items` test from
`cc_bindings_from_rs/bindings.rs` -->

For the following Rust type:

```rust
pub enum Point {
    Cartesian(f32, f32),
    Polar{ dist: f32, angle: f32 },
}
```

Crubit will generate the following bindings:

```cpp
struct alignas(4) Point final {
    public:
        // The Rust type has no `Default` impl.
        Point() = delete;

        // The Rust type is not `Copy`.
        Point(const Point&) = delete;
        Point& operator=(const Point&) = delete;

        // All non-`Drop` Rust types are trivially-movable.
        Point(Point&&) = default;
        Point& operator=(Point&&) = delete;

        // The Rust type has no `Drop` impl,
        // nor requires custom drop glue.
        ~Point() = default;
    private:
        ...
};
```

Note that the generated C++ bindings are always opaque. In particular, the C++
side doesn't currently have any direct visibility into the discriminant nor
fields of the variants of the Rust enum. Nevertheless, the bindings should cover
methods and trait of the Rust enum - for example:

*   mapping static methods from Rust to non-member methods in C++
*   mapping `Default` trait impl from Rust to the default constructor in C++
    (this bullet item is WIP - see b/258249980)

TODO: Adding support for constructing tagged union values is tracked in
b/280861833.
