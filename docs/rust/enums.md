# C++ bindings for Rust `enum`s

A Rust `enum` is mapped to a C++ `struct` with a nested struct for each variant.

If an `enum` is not [C++ movable](movable_types.md), then it will be passed
by-value via the [`rs::Movable`](movable_types.md#movable_wrapper) wrapper type.
See [Movable Types](movable_types.md).

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
`enum` declaration does not exist, and it is [undefined
behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
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

If an `enum` is not [C++ movable](movable_types.md), its C++ move constructor
and move assignment operator will be `= delete`d, and it will be passed by-value
via the [`rs::Movable`](movable_types.md#movable_wrapper) wrapper type. See
[Movable Types](movable_types.md).

## Enums with payload

Each variant of a Rust `enum` may be a unit variant or carry a tuple or struct
payload. For an enum `E`, Crubit generates a nested struct `E::Variant` for each
variant:

*   **Constructing variants**: Call `E::Variant::Make(...)` (or
    `E(std::in_place_type<E::Variant>, ...)`), passing the variant's fields in
    declaration order. Variants are built directly in `E`'s storage and cannot
    be constructed, copied, or moved standalone (`E::Variant(...)` does not
    compile).
*   **Inspecting variants and payloads**
    (`#include "support/rs/enum.h"`,
    `//support/rs:enum`):
    *   `rs::holds_alternative<E::Variant>(e)` returns `true` if `e` holds
        `Variant`.
    *   `rs::get_if<E::Variant>(&e)` takes a nullable pointer (`const E*` or
        `E*`) and returns a pointer (`const E::Variant*` or `E::Variant*`) to
        the active variant if the pointer is non-null and `e` holds `Variant`,
        or `nullptr` otherwise. Tuple variant fields are named `__field0`,
        `__field1`, etc., and struct variant fields keep their Rust names.
    *   `rs::discriminant(e)` returns an opaque, trivially copyable
        `rs::Discriminant<E>` (analogous to Rust's `std::mem::Discriminant<E>`)
        that identifies which variant is active, but not the payload: two values
        holding the same variant compare equal. It supports equality comparison
        (`==`/`!=`) only: it does not expose the underlying integer, is not
        ordered, and cannot be used in a `switch` statement.
    *   `rs::get_unchecked<E::Variant>(&e)` takes a non-null pointer (`const E*`
        or `E*`) and returns a non-null pointer (`const E::Variant*` or
        `E::Variant*`) to the active variant. It does not check the tag when
        `NDEBUG` is defined; otherwise it checks it at runtime with
        `CRUBIT_DCHECK`. Passing `nullptr`, or calling it when `e` holds a
        different variant, is undefined behavior.
*   **`constexpr` support**:
    *   `E::Variant::Make(...)`, `E(std::in_place_type<E::Variant>, ...)`, and
        `rs::get_unchecked<E::Variant>(&e)` are `constexpr`. Constructing a
        variant in a constant expression requires `E` to have no `Drop` impl of
        its own (drop glue from other variants' fields is fine), the constructed
        variant to be trivially destructible (no drop glue on any of its field
        types), and each field's Rust type to be one Crubit recognizes as
        `constexpr`-movable: primitives, references, raw/function pointers,
        `NonNull`, `Pin`, `#[cpp_enum]` enums, arrays and C++-aggregate structs
        whose elements/fields recursively qualify, and non-`#[repr(C)]` enums
        with at least one qualifying variant. Imported C++ types, tuples,
        non-aggregate structs, `#[repr(C)]` enums, and unions are excluded even
        when their C++ types are literal types.
    *   `rs::discriminant`, `rs::holds_alternative`, and `rs::get_if` are
        runtime-only (not `constexpr`): they decode the tag from the object
        representation, which is not allowed in constant expressions. Making
        them `constexpr` needs C++26 `std::is_within_lifetime`, which not every
        supported compiler provides.

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
