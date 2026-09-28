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


Which can be used like this:

```
{{ #include ../../examples/cpp/trivial_struct/main.rs }}
```
<!--  function:main -->


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
example, base class subobjects are not exposed as fields, so the space of the
object occupied by a base class will instead be this opaque blob of bytes, with
no comment. (See [Base classes](#inheritance) for what you can do with base
classes instead.)

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

## Base classes {#inheritance}

Rust has no inheritance, so a derived class and its base classes become
unrelated Rust structs. The base class subobject is an
[opaque blob](#opaque_fields), so base class fields aren't accessible through
the derived struct. With `oo_casting`, Rust code can
[upcast](#inherited_methods) to the base class and read the fields there.

### Inherited methods {#inherited_methods}

By default, methods that a class inherits from its base classes are **not** part
of the derived class's bindings, and no comment is emitted about them. They
still exist on the base class's bindings.

```c++
struct Base {
  bool IsValid() const;
};
struct Derived : Base {};
```

Here, `Base::IsValid()` has bindings, but `Derived` has no `IsValid()` method in
Rust.

To get inherited methods, add `//features:oo_casting` to the
`aspect_hints` of the `cc_library` that defines the derived class. This is an
[experimental](../overview/status.md) feature. With it:

*   The derived struct gets the public methods, including static methods, of its
    public base classes, so `derived.IsValid()` works in Rust. This only covers
    base classes defined in the same `cc_library` as the derived class, and
    leaves out methods whose name comes from more than one base class.
*   The derived struct implements `oops::Inherits<Base>` for each public base
    class, including base classes from other `cc_library` targets. Rust code can
    then use the `oops::Upcast` trait to convert a `&Derived`, `*const Derived`,
    or `*mut Derived` to the base class type. For example, to call a method of a
    base class in another `cc_library`, upcast first:

    ```rust
    use oops::Upcast as _;

    // The type annotation is required.
    let base: &Base = unsafe { derived.upcast() };
    ```

    For a `virtual` base class, use `oops::VirtualUpcast` on a raw pointer
    instead. Rust code that uses these traits depends on
    `//support:oops`.

The tests in
[`rs_bindings_from_cc/test/struct/inheritance/`](/rs_bindings_from_cc/test/struct/inheritance/)
show both.

Without `oo_casting`, add a method to the derived class in C++ that forwards to
the base class method, or a function that returns the object as a pointer to its
base class.

`using Base::Method;` declarations in the derived class don't get bindings, and
produce a "Function aliases are not yet supported" error comment. With
`oo_casting`, the method is usually inherited anyway. It's missing only if the
`using` declaration makes a method of a non-public base class accessible, or
brings back a base class method that a derived class method of the same name
hides.

### Virtual methods {#virtual}

Calling a C++ virtual method from Rust works like calling any other method, and
dispatches virtually. This includes pure virtual methods. Because a class with
virtual methods is not [Rust-movable](#rust_movable), non-`const` methods take
`self: Pin<&mut Self>`.

An abstract class (one with pure virtual methods that aren't overridden) gets
bindings for its methods but no constructor, so Rust code works with it through
references and pointers to objects created elsewhere. A subclass that overrides
the methods gets bindings for its overrides and can be constructed. The methods
it doesn't override are [inherited methods](#inherited_methods).

Overriding a C++ virtual method in Rust, or otherwise implementing a C++
interface in Rust, is not supported. The workaround is a C++ subclass that
forwards to a Rust type; see
[`examples/cpp/virtual/`](/examples/cpp/virtual/).

## Attributes {#attributes}

Crubit does not support most attributes on structs and their fields. If a struct
is marked using any attribute other than alignment or
`ABSL_ATTRIBUTE_TRIVIAL_ABI`, it will not receive bindings. If a field is marked
using any other attribute, it will be replaced with a private opaque blob.
