# Types

## Overview

<internal link>/overview/status#types outlines the current and future status of
Crubit's type support.

In brief, Crubit supports:

*   Primitive types (<internal link>/types/primitive), such as `float` or `i32`.
*   Pointer types (<internal link>/types/pointer), such as `float*` or `*const i32`,
    including function pointers.
*   User-defined types, with some language-specific rules and restrictions. (See
    <internal link>/cpp and <internal link>/rust).

## ABI-Compatibility

Certain references to C++ or Rust types will not receive Crubit bindings.
Some types may only be usable in certain locations due to current Crubit
limitations, inherent properties of the type, or both. Supported types fall into
one of three categories ranging from "most widely supported" to
"most restricted":

* **ABI-compatible**: these types have a C-ABI-equivalent representation which
    can be used anywhere a value of this type is expected from both C++ and
    Rust.
* **Layout-compatible**: these types have equivalent in-memory representations
    in C++ and Rust but cannot be represented using standard C ABI. These types
    will only be usable as by-value function arguments if they are C++-movable.
    For example, `Box<i32>` is not C++-movable because it has no `nullptr` /
    moved-from representation.
* **Bridged**: these types may have different in-memory representations in C++
    and Rust, and so can only be passed by-value between the two languages.
    Examples include Rust tuples, which are bridged by-value into C++
    `std::tuple`.

| Level of Support            | Example             | Pass by-reference |         Pass by-value        |        Return by-value       | Fields | In Function Pointer Types |
|-----------------------------|---------------------|:-----------------:|:----------------------------:|:----------------------------:|:------:|:-------------------------:|
| ABI Compatible              | `i32`               |         Y         |               Y              |               Y              |    Y   |             Y             |
| Layout-compatible C++ type  | `absl::string_view` |         Y         | if trivially relocatable[^1] | if trivially relocatable[^2] |    Y   |             N             |
| Layout-compatible Rust type | `UserDefinedStruct` |         Y         |      if C++ movable[^3]      |               Y              |    Y   |             N             |
| Bridged                     | `(i32, i32)`        |         N         |               Y              |               Y              |    N   |             N             |

[^1]: See <internal link>/cpp/classes_and_structs#trivially_relocatable
[^2]: See <internal link>/cpp/classes_and_structs#trivially_relocatable
[^3]: See <internal link>/rust/movable_types

NOTE: All primitive and pointer types are ABI-compatible. However, due to
b/369895805, all non-bridged user-defined types are **only** layout-compatible.

In the following examples, `foo` receives bindings, but `bad_foo` will not
receive bindings, because while the types it uses in its function signature are
supported by Crubit, they are not supported in this particular context.

<section class="tabs" markdown=1>

#### C++ {.new-tab}

```c++ {.good}
void foo(int32_t);
void foo(void (*)(int32_t));
void foo(Status);
```

```c++ {.bad}
struct LayoutCompatibleType {
    UnsupportedType field;
    // or [[no_unique_addres]] int field; or...
};

// foo cannot receive bindings, because the function pointer type
// does not work with non-ABI-compatible types
void bad_foo(void (*)(LayoutCompatibleType));
```

```cpp {.bad}
// foo cannot receive bindings, because bridged types cannot be passed
// by reference
void bad_foo(const Status&);
```

#### Rust {.new-tab}

```rust {.good}
pub fn foo(_: i32) {}
pub fn foo(_: fn(i32)) {}
pub fn foo(_: Status) {}
```

```rust {.bad}
struct LayoutCompatibleType {
    field: UnsupportedType
}

// foo cannot receive bindings, because the function pointer type
// does not work with non-ABI-compatible types
pub fn bad_foo(_: fn(LayoutCompatibleType)) {}
```

```rust {.bad}
// foo cannot receive bindings, because bridged types cannot be passed
// by reference
fn bad_foo(_: &Status) {}
```

</section>

## Bidirectionality

Usually, the mapping of types between languages is bidirectional. For example, a
C++ function which returns an `int32_t` will become a Rust function returning an
`i32`, and vice versa. In some sense, an `i32` **is** an `int32_t`.

However, in other cases, the mapping is not reversible. C++ and Rust have types
or aliases that the other language does not. For example, `isize` becomes
`intptr_t`, but `intptr_t` is (on some platforms) the same type as `int64_t`,
and so `intptr_t` becomes `i64`.
