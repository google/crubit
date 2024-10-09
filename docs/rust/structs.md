# C++ bindings for Rust structs

A Rust `struct` is mapped to a C++ `class`/`struct` with the same fields. If any
field cannot be represented in C++, the struct itself will still have bindings,
but [the relevant field will be private](#opaque_fields).

To receive C++ bindings, the `struct` must be movable in C++. See
[Movable Types](movable_types).

## Example

Given the following Rust module:

```live-snippet
cs/file:examples/rust/struct/example.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

Crubit will generate the following bindings:

<!-- Note: Kythe currently indexes this as class `CRUBIT_INTERNAL_RUST_TYPE` because it doesn't have a build rule. -->

```live-snippet
cs/file:examples/rust/struct/example_generated.h class:CRUBIT_INTERNAL_RUST_TYPE|Struct
```

## Fields {#fields}

The fields on the C++ class are the corresponding Rust types:

*   If the Rust field has [primitive type](../types/primitive), then the C++
    field uses the corresponding C++ type.
*   Similarly, if the Rust field has [pointer type](../types/pointer), then the
    C++ field has the corresponding C++ pointer type.
*   If the field has a user-defined type, such as a struct or [enum](enums),
    then the bindings for the function use the bindings for that type.

### Unsupported fields {#opaque_fields}

Fields that do not receive bindings are made private, and replaced with an
opaque blob of maybe-uninitialized bytes, as well as a comment in the generated
source code explaining why the field could not receive bindings. For example,
since `String` is not supported, the space of the object occupied by a `String`
field will instead be this opaque blob of bytes:

```rust {.bad}
// Rust: `my_field` is some unsupported type, such as `String`
pub my_field: String,
```

```c++ {.bad}
// C++: `my_field` becomes `private`, and its type is replaced by bytes.
private: unsigned char my_field[24]
```

Specifically, the following subobjects are hidden and replaced with opaque
blobs:

*   Non-public fields (`private` or `pub(...)` fields).
*   Fields that implement `Drop`.
*   Fields whose type does not have bindings.
*   Fields that have an unrecognized or unsupported attribute.

## C++ movable {#cpp_movable}

To receive C++ bindings, the `struct` must be movable in C++. See
[Movable Types](movable_types).
