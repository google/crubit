# C++ bindings for Rust structs

A Rust `struct` is mapped to a C++ `class`/`struct` with the same fields. If any
field cannot be represented in C++, the struct itself will still have bindings,
but [the relevant field will be private](#opaque_fields).

To receive C++ bindings, the `struct` must be movable in C++. See
[Movable Types](movable_types.md).

## Example

Given the following Rust module:

```
{{ #include ../../examples/rust/struct/example.rs }}
```
<!--  class:Struct -->


Crubit will generate the following bindings:

<!-- Note: Kythe currently indexes this as class `CRUBIT_INTERNAL_RUST_TYPE` because it doesn't have a build rule. -->

```
{{ #include ../../examples/rust/struct/example_generated.h }}
```
<!--  class:CRUBIT_INTERNAL_RUST_TYPE|Struct -->


## Fields {#fields}

The fields on the C++ class are the corresponding Rust types:

*   If the Rust field has [primitive type](../types/primitive.md), then the C++
    field uses the corresponding C++ type.
*   Similarly, if the Rust field has [pointer type](../types/pointer.md), then
    the C++ field has the corresponding C++ pointer type.
*   If the field has a user-defined type, such as a struct or [enum](enums.md),
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
*   Fields whose type does not have bindings.
*   Fields that have an unrecognized or unsupported attribute.

## Aggregate initialization {#aggregates}

Qualifying Rust structs are generated as C++ **aggregates** (satisfying
[`std::is_aggregate_v<T>`](https://en.cppreference.com/w/cpp/types/is_aggregate)),
which enables C++ aggregate initialization such as designated initializers
(`MyStruct{.a = 1, .b = 2}`) and braced initialization (`MyStruct{1, 2}`).

A Rust struct is generated as a C++ aggregate if:

1.  All of its fields are public (`pub`) and supported by Crubit.
2.  It is not marked `#[non_exhaustive]`.
3.  It does not implement `Drop`.
4.  At most one field requires drop glue, **or** the struct is annotated with
    `#[crubit_annotate::field_drop_order_does_not_matter]`.
5.  Any field requiring drop glue is movable in C++.
6.  `Default` is not implemented manually (i.e. `Default` is either derived via
    `#[derive(Default)]` or not implemented at all).

### Field Drop Order and `#[crubit_annotate::field_drop_order_does_not_matter]`

In Rust, struct fields are dropped in definition order (first to last). In C++,
aggregate members are destroyed in reverse definition order (last to first).

When at most one field requires drop glue, destruction order is trivial and
matches between Rust and C++. However, when multiple fields require drop glue,
the drop order in C++ will be the reverse of Rust's drop order. To prevent
unintended semantic differences, Crubit generates such structs as non-aggregates
(with custom C++ destructors calling into Rust) by default.

If the drop order of the fields does not matter, you can annotate the struct
with `#[crubit_annotate::field_drop_order_does_not_matter]` from
`//support:crubit_annotate` to opt into C++ aggregate
generation:

```rust
use crubit_annotate::field_drop_order_does_not_matter;

#[field_drop_order_does_not_matter]
pub struct Config {
    pub name: MyDropType1,
    pub buffer: MyDropType2,
}
```

### Manual `Default` Implementation

If a struct implements `Default` manually (`impl Default for MyStruct { ... }`),
it is generated as a non-aggregate. This allows Crubit to generate an explicit
C++ default constructor (`MyStruct()`) that calls Rust's `Default::default()`,
preserving any custom initial values defined in Rust.

Structs using `#[derive(Default)]` remain C++ aggregates, as derived `Default`
initializes primitive fields to zero/false, matching C++ value initialization.

### Differences between Aggregate and Non-Aggregate Structs

*   **Direct Member Variables**: C++ aggregates expose fields as direct member
    variables (`Type field_name;`) without anonymous union wrappers or padding
    fields.
*   **No User-Declared Constructors**: C++ aggregates do not have user-declared
    constructors (such as converting constructors or tuple constructors).
*   **Value Initialization vs `Default::default()`**: In C++, value-initializing
    an aggregate (`MyStruct s{};`) zero-initializes primitive fields according to
    C++ rules. If a struct derives `Default` or does not implement `Default`,
    zero-initialization in C++ matches Rust derived default semantics.

## C++ movable {#cpp_movable}

To receive C++ bindings, the `struct` must be movable in C++. See
[Movable Types](movable_types.md).

## `CRUBIT_INTERNAL_RUST_TYPE` annotation {#crubit_internal_rust_type}

You may notice that the generated C++ structs are annotated with the
`CRUBIT_INTERNAL_RUST_TYPE` macro. This annotation instructs Crubit
(specifically `rs_bindings_from_cc`) to disable automated bindings for this C++
type, and instead map all C++ uses of the type back to the existing Rust type.
This ensures that a Rust struct passed to C++ and then back to Rust resolves to
the original Rust type rather than a newly generated one.

While Crubit generates this annotation automatically for Rust-to-C++ bindings,
you can also apply it manually on your own C++ types if you want them to map to
an existing Rust type:

```c++
struct CRUBIT_INTERNAL_RUST_TYPE("char") char_ {
    std::uint32_t c;
};
```

### Template Arguments and Interpolation

For C++ templates, you can use `{}` interpolation syntax within the Rust type
string to substitute template arguments:

```c++
template <typename T>
struct CRUBIT_INTERNAL_RUST_TYPE("RustType<{}>", T) CppType {
    T* value;
};
```

This ensures that a C++ instantiation like `CppType<int>` maps correctly to
`RustType<i32>` in Rust.

Importantly, this interpolation syntax allows you to express Rust generic
parameters that have no direct C++ equivalent, such as lifetimes or default
generic arguments. For example:

```c++
template <typename T>
struct CRUBIT_INTERNAL_RUST_TYPE("RustType<'static, {}>", T) CppType {
    T* value;
};
```

Const generics arguments can also be provided with
`crubit::rust_type::Const<N>`:

```c++
template <typename T>
struct CRUBIT_INTERNAL_RUST_TYPE(
    "RustType<'static, {}, {}>",
    T,
    crubit::rust_type::Const<123>,
) CppType {
    T* value;
};
```
