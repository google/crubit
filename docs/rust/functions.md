# C++ bindings for Rust functions

C++ code can call functions defined in Rust, provided that the parameter and
return types are supported by Crubit:

*   If a parameter or return type is a
    [fundamental type](../cpp/fundamental_types.md), then the bindings for the
    function use the corresponding Rust type.
*   Similarly, if a parameter or return type is a
    [pointer type](../cpp/pointer_types.md), then the bindings for the function
    use the corresponding Rust pointer type.
*   If the type is a user-defined type, such as a [struct](struct.md) or
    [enum](enums.md), then the bindings for the function use the bindings for
    that type.

As a special case, functions also support reference parameters to supported
types, with some restrictions to ensure safety. See [References](#references).

## Example

<iframe src="https://crubit-explorer-749456143212.us-central1.run.app/embed#code=eyJ2IjoxLCJ0b29sIjoiY2NfYmluZGluZ3NfZnJvbV9ycyIsImZpbGVzIjpbeyJuYW1lIjoiaW5wdXQucnMiLCJjb250ZW50IjoicHViIGZuIGFkZF90d29faW50ZWdlcnMoeDogaTMyLCB5OiBpMzIpIC0-IGkzMiB7XG4gICAgeCArIHlcbn0ifV19" width="100%" height="450" frameborder="0"></iframe>

## `unsafe` functions

C++ does not have an `unsafe` marker at this time. **In the future**, Crubit may
introduce a way to mark `unsafe` functions to help increase the reliability of
C++ callers.

## References

In general, Rust references are not exposed to C++. However, some Rust functions
which accept reference parameters do get mapped to C++ functions accepting C++
references:

*   All references must have an unbound parameter lifetime – not `'static`, for
    example.
*   Only the parameter itself can be a reference type. References to references,
    vectors of references, etc. are still unsupported.
*   If there is a `mut` reference parameter, it is the **only** reference
    parameter.

This set of rules is intended to describe a safe subset of Rust functions, which
do not introduce substantial aliasing risk to a mixed C++/Rust codebase.

For example, the following Rust functions will receive C++ bindings, and can be
called from C++:

```rust {.good}
fn foo(&self) {}
fn foo(_: &mut i32) {}
fn foo(_: &i32, _: &i32) {}
```

However, none of the below will receive bindings:

```rust {.bad}
fn foo(_: &'static i32) {}  // 'static lifetime is bound
fn foo(_: &&i32) {}  // Reference in non-parameter type
fn foo(_: &mut i32, _: &i32) {}  // More than one reference, one of which is mut
fn foo(_: &'a i32) {}  // 'a is not a lifetime parameter of `foo`
```

Returned references are still not supported, and references which are bound to
some lifetime (e.g. `'static`) are also still not supported.

If you wish to accept more than one reference/pointer in C++, a raw pointer
(`*const T`, `*mut T`) can be used instead. However, all of the usual `unsafe`
caveats apply.

## Slice parameters

Rust slice parameters (`&[T]`, `&str`) map to `rs_std::SliceRef<const T>` and
`rs_std::StrRef` in C++.

C++ callers can pass string literals (`"hello"`) or `std::string_view` directly
to satisfy string and byte-slice parameters.

```rust {.good}
fn process_text(_: &str) {}     // C++ callers can pass "hello" or std::string_view
fn process_bytes(_: &[u8]) {}   // C++ callers can pass "hello", std::string_view, or std::span<const uint8_t>
```

However, mutable slices cannot be constructed from string literals:

```rust {.bad}
fn process_mut_bytes(_: &mut [u8]) {}  // Cannot pass string literals to mutable byte slices
```

*   **`&str`**: Maps to `rs_std::StrRef` (`crubit/support/rs_std/str_ref.h`).
    Converts from string literals and `std::string_view`.
*   **`&[u8]`**: Maps to `rs_std::SliceRef<const uint8_t>`
    (`crubit/support/rs_std/slice_ref.h`). Converts from string literals,
    `std::string_view`, `std::vector<uint8_t>`, and `std::array<uint8_t, N>`.

## Generic functions

Crubit doesn't support generating bindings for *arbitrary* generic functions.
Nevertheless, a small set of generic type parameters is supported by using
canonical substitutions which are defined as follows:

Generic constraints | Canonical substitution
------------------- | ----------------------
`T: AsMut<U>`       | `&mut U`
`T: AsRef<U>`       | `&U`
`T: Into<U>`        | `U`
