# Rust bindings for C++ classes and structs

A C++ `class` or `struct` is mapped to a Rust `struct` with the same fields. If
any subobject of the class cannot be represented in Rust, the class itself will
still have bindings, but
[the relevant subobject will be private](#opaque_fields).

## Example

Given the following C++ header:

```
{{ #include ../../examples/cpp/trivial_struct/example.h }}
```
<!--  class:Position -->


Crubit will generate a struct with the same layout:

```
{{ #include ../../examples/cpp/trivial_struct/example_generated.rs }}
```
<!--  class:Position -->


For an example of a Rust-movable class with a destructor, see
[examples/cpp/trivial_abi_struct/](https://github.com/google/crubit/tree/main/examples/cpp/trivial_abi_struct/).

## Fields {#fields}

The fields on the Rust struct type are the corresponding Rust types:

*   If the C++ field has [primitive type](../types/primitive.md), then the Rust
    field uses the corresponding Rust type.
*   Similarly, if the C++ field has [pointer type](../types/pointer.md), then
    the Rust field has the corresponding Rust pointer type.
*   If the field has a user-defined type, such as a
    [class type](classes_and_structs.md) or [enum](enums.md), then the bindings
    for the function use the bindings for that type.

### Unsupported fields {#opaque_fields}

Subobjects that do not receive bindings are made private, and replaced with an
opaque blob of `[MaybeUninit<u8>; N]`, as well as a comment in the generated
source code explaining why the subobject could not receive bindings. For
example, since inheritance is not supported, the space of the object occupied by
a base class will instead be this opaque blob of bytes.

Specifically, the following subobjects are hidden and replaced with opaque
blobs:

*   Base class subobjects
*   Non-`public` fields (`private` or `protected` fields)
*   Fields that have nontrivial destructors
*   Fields whose type does not have bindings
*   Fields that have any unrecognized attribute, including `no_unique_address`

A Rust struct with opaque blobs is ABI-incompatible with the C++ struct or class
that it corresponds to. As a consequence, if the struct is used for FFI outside
of Crubit, it should not be passed by value. Within Crubit, it can't be passed
by value in [function pointers](../types/pointer.md#function), but can otherwise
be used as normal.

<span id="trivially_relocatable"></span>

## Rust-movable classes {#rust_movable}

The easiest C++ classes to work with are "Rust-movable", meaning they support
being relocated in memory using `memcpy` without running the move constructor.
When passed and returned by value, Rust-movable classes will use the direct
type. For example, `T Identity(T)` becomes `pub fn Identity(_: T) -> T`.

If it is non-Rust-movable, then Crubit will instead use in-place initialization
types, `Ctor`, and requires a bit of syntactic overhead when moving objects
around. The same function might instead become `pub fn Foo(_: Ctor![T]) ->
Ctor![T]`.

(For an introduction to `Ctor![T]`, see
crubit.rs/types/non_rust_movable/intro_short.)

Types are considered Rust-movable by default, meaning they can be relocated
using `memcpy`. If the type defines a destructor or copy/move constructor, then
it requires a special annotation to be considered Rust-movable:
[`ABSL_ATTRIBUTE_TRIVIAL_ABI`](https://github.com/abseil/abseil-cpp/blob/master/absl/base/attributes.h#:~:text=ABSL_ATTRIBUTE_TRIVIAL_ABI).
If it has a non-Rust-movable field or base class, then it is not Rust-movable.

It can be worth putting some effort into designing the type to be Rust-movable.
crubit.rs/cpp/cookbook#rust_movable describes, in more detail, how to go about
this.

Some examples of Rust-movable types:

*   any primitive type (integers, character types, floats, etc.)
*   raw pointers
*   `string_view`
*   [`struct tm`](https://en.cppreference.com/w/cpp/chrono/c/tm), or any other
    type in the C standard library
*   `unique_ptr` and `shared_ptr`, in the Clang unstable ABI.
*   `absl::Status`

Some examples of types that are **not** Rust-movable:

*   (For now) `std::string`, `std::vector`, and other nontrivial standard
    library types.
*   (For now) `absl::flat_hash_map`, `absl::AnyInvocable`, and other nontrivial
    types used throughout the C++ ecosystem, even outside the standard library.
*   `absl::Mutex`, `absl::Notification`, and other non-movable types.

## Attributes {#attributes}

Crubit does not support most attributes on structs and their fields. If a struct
is marked using any attribute other than alignment or
`ABSL_ATTRIBUTE_TRIVIAL_ABI`, it will not receive bindings. If a field is marked
using any other attribute, it will be replaced with a private opaque blob.
