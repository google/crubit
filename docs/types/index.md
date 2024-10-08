# Types

[TOC]

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

Some types have restrictions on where they can be used. We can categorize types
into three categories:

*   **ABI-compatible**: the type can be used by value or by pointer, in fields,
    functions, function pointers. All primitive and pointer types are
    ABI-compatible.
*   **Layout-compatible**: the type can be used anywhere an ABI-compatible type
    can be used, except that it cannot be accepted or returned by value in
    functions in the following circumstances:
    *   if the type is not movable in C++, or
    *   if the FFI is performed manually, without using Crubit, or
    *   the function is a function pointer.
*   **Bridged**: the type can only be passed or returned directly by value. It
    cannot be used by pointer, in struct fields, or in function pointers in any
    way.

NOTE: All primitive and pointer types are ABI-compatible. However, due to
b/369895805, all non-bridged user-defined types are **only** layout-compatible.

For example, Rust's `i32` is an ABI-compatible type: it maps to C++'s `int32_t`,
and can even be passed by value as an `int32_t` in function pointers.

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
