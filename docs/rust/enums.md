# C++ bindings for Rust `enum`s

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
