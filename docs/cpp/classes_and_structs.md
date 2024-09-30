# Rust bindings for C++ classes and structs

A C++ `class` or `struct` is mapped to a Rust `struct` with the same fields. If
any subobject of the class cannot be represented in Rust, the class itself will
still have bindings, but
[the relevant subobject will be private](#opaque_fields).

To have bindings, the class must be
["trivially relocatable"](#trivially_relocatable). For example, any trivial or
"POD" class is trivially relocatable.

## Example

Given the following C++ header:

```live-snippet
cs/file:examples/cpp/trivial_struct/example.h class:Position
```

Crubit will generate a struct with the same layout, as well as some debug
assertions to ensure the bindings are correct.

```live-snippet
cs/file:examples/cpp/trivial_struct/example_generated.rs content:^([^/\n])([^!\n]|$)[^\n]*
```

For an example of a trivially-relocatable class with a destructor, see
[examples/cpp/trivial_abi_struct/](http://examples/cpp/trivial_abi_struct/).

## Fields {#fields}

The fields on the Rust struct type are the corresponding Rust types:

*   If the C++ field has [primitive type](../types/primitive), then the Rust
    field uses the corresponding Rust type.
*   Similarly, if the C++ field has [pointer type](../types/pointer), then the
    Rust field has the corresponding Rust pointer type.
*   If the field has a user-defined type, such as a
    [class type](classes_and_structs) or [enum](enums), then the bindings for
    the function use the bindings for that type.

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
by value in [function pointers](../types/pointer#function), but can otherwise be
used as normal.

## Trivially relocatable classes {#trivially_relocatable}

To receive Rust bindings, a type must be
["trivially relocatable"](https://clang.llvm.org/docs/LanguageExtensions.html#:~:text=__is_trivially_relocatable).
This is the C++ version of saying a class is "Rust-movable": the class can be
"teleported" in memory during its lifetime, as if by using `memcpy` and then
discarding the old location without running any destruction logic. This means
that it can be present in Rust using normal objects and pointers and references,
without using `Pin`.

For example, a `string_view` is trivially relocatable. In fact, every trivially
copyable type is trivially relocatable.

However, unlike Rust, many types in C++ are **not** trivially relocatable. For
example, a `std::string` might be implemented using the "short string
optimization", in a fashion similar to this:

```c++
class String {
    union {
        size_t length;
        char inline_data[sizeof(length)];
    };
    char* data; // either points to `inline_data`, or the heap.
  public:
    size_t size() {
        if (data == (char*)this) {
            return strlen(data);
        } else {
            return length;
        }
    }
    // ...
};
```

This class is self-referential: the `data` pointer may point to `inline_data`,
which is inside the object itself. If we bitwise copy the object to a new
location, as in a "Rust move" or as with `memcpy`, then the `data` pointer will
remain bitwise identical, and point into the **old** object. It becomes a
dangling pointer!

C++ allows self-referential types. In C++, fields can and often do point at
other fields, because assignment is overloadable: the assignment operator can be
modified to, when copying or moving the string, also "fix up" the `data` pointer
so that it points to the *new* location in the new object, instead of dangling.

Rust does not do this. In Rust, assignment is always a "trivial relocation" --
assignment runs *no* code when copying or moving an object, and copies the bytes
as they are. This would break on the `String` type defined above, or any other
self-referential type.

Unfortunately, any class with a user-defined copy/move operation or destructor
*might* be self-referential, and so by default they are not trivially
relocatable. If a class has a user-defined destructor or copy/move
constructor/assignment operator, and "should be" trivially relocatable, it must
explicitly declare that it is safe to trivially relocate, using the attribute
[`ABSL_ATTRIBUTE_TRIVIAL_ABI`](https://github.com/abseil/abseil-cpp/blob/master/absl/base/attributes.h#:~:text=ABSL_ATTRIBUTE_TRIVIAL_ABI).
This attribute allows a class to be trivially relocated, even though it defines
an operation that would ordinarily disable trivial relocation.

For example, in the unstable libc++ ABI we use within Google, a `unique_ptr<T>`
is trivially relocatable, because it applies `ABSL_ATTRIBUTE_TRIVIAL_ABI`. This
is safe to do, for `unique_ptr`, because its exact location in memory does not
matter, and paired move/destroy operations can be replaced with trivial
relocations.

### Requirements

The exact requirements for a class to be trivially relocatable are subject to
change, because they are still being defined within Clang and within the C++
standard. But at the least:

*   Any
    [trivially copyable](https://en.cppreference.com/w/cpp/language/classes#Trivially_copyable_clas)
    type is also trivially relocatable.
*   Any `class` or `struct` type with only trivially relocatable fields and base
    classes is trivially relocatable, unless:
    *   it is not `ABSL_ATTRIBUTE_TRIVIAL_ABI` and defines a copy/move
        constructor, copy/move assignment operator, or destructor, or,
    *   it is otherwise nontrivial, e.g., from defining a `virtual` member
        function.

Some examples of trivially relocatable types:

*   any primitive type (integers, character types, floats, etc.)
*   raw pointers
*   `string_view`
*   [`struct tm`](https://en.cppreference.com/w/cpp/chrono/c/tm), or any other
    type in the C standard library
*   `unique_ptr`, in the Clang unstable ABI.
*   `absl::Status`

Some examples of types that are **not** trivially relocatable:

*   (For now) `std::string`, `std::vector`, and other nontrivial standard
    library types.
*   (For now) `absl::flat_hash_map`, `absl::AnyInvocable`, and other nontrivial
    types used throughout the C++ ecosystem, even outside the standard library.
*   `absl::Mutex`, `absl::Notification`, and other non-movable types.

## Attributes {#attributes}

Crubit does not support most attributes on structs and their fields. If a struct
is marked using any attribute other than alignment or
`ABSL_ATTRIBUTE_TRIVIAL_ABI`, it will not receive bindings. If a field is marked
using any attribute, it will be replaced with a private opaque blob.
