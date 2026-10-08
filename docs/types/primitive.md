# Primitive types

Crubit maps primitive types[^terminology] to the direct equivalent in the other
language. For example, C++ `int32_t` is Rust `i32`, C++ `int` is Rust
`ffi_11::c_int`, C++ `double` is Rust `f64`, and so on.

Exceptions:

*   **C++:** There is no mapping for the currently-unsupported types
    `nullptr_t`, `char8_t`, and `wchar_t`.
*   **Rust:** There is no mapping for the currently-unsupported `char` and `str`
    types, the never (`!`) type except as a return type, and `i128` / `u128`
    when calling Rust from C++.

For more information, see [Unsupported types](#unsupported)

## Bidirectional type mapping {#bidirectional}

The following map is bidirectional. If you call a C++ interface from Rust using
Crubit, then `int32_t` in C++ becomes `i32` in Rust. Vice versa, if you call a
Rust interface from C++ using Crubit, `i32` in Rust becomes `int32_t` in C++.

C++                  | Rust
-------------------- | -------------------------------------------------------
`void`               | `()` as a return type, `::ffi_11::c_void` otherwise.
`int8_t`             | `i8`
`int16_t`            | `i16`
`int32_t`            | `i32`
`int64_t`            | `i64`
`intptr_t`           | `isize`
`uint8_t`            | `u8`
`uint16_t`           | `u16`
`uint32_t`           | `u32`
`uint64_t`           | `u64`
`uintptr_t`          | `usize`
`bool`               | `bool`
`double`             | `f64`
`float`              | `f32`
`char`               | `::ffi_11::c_char` [^char]
`signed char`        | `::ffi_11::c_schar`
`unsigned char`      | `::ffi_11::c_uchar`
`short`              | `::ffi_11::c_short`
`unsigned short`     | `::ffi_11::c_ushort`
`int`                | `::ffi_11::c_int`
`unsigned int`       | `::ffi_11::c_uint`
`long`               | `::ffi_11::c_long`
`unsigned long`      | `::ffi_11::c_ulong`
`long long`          | `::ffi_11::c_longlong`
`unsigned long long` | `::ffi_11::c_ulonglong`
`rs_std::char_`      | `char`

## One-way type mapping {#one_way}

The types below are mapped in only one direction, but do not round trip back to
the original type. For example, `size_t` maps to `usize`, but `usize` maps to
`uintptr_t`.

### C++ to Rust {#cpp_to_rust}

The following C++ types become the following Rust types, but not vice versa:

C++                 | Rust
------------------- | -----------------
`ptrdiff_t`         | `isize`
`size_t`            | `usize`
`char16_t`          | `u16`
`char32_t`          | `u32` [^char32_t]
`__int128`          | `i128` [^int128]
`unsigned __int128` | `u128`

### One-way mapping of Rust to C++ types {#rust_to_cpp}

The following Rust types become the following C++ types, but not vice versa:

Rust              | C++
----------------- | ------
`!` (return type) | `void`

## Unsupported types {#unsupported}

Bindings for the following types are not supported at this point:

### C++

*   `nullptr_t` and `char8_t` have not yet been implemented.
*   b/283268558: `wchar_t` is currently unsupported, for portability reasons.

### Rust

*   b/254507801: `!` has not yet been implemented except for return types.
*   b/254094650: `i128` and `u128` are not yet supported when calling Rust from
    C++.

[^terminology]: Rust calls these types
    [primitive types](https://doc.rust-lang.org/reference/types.html), while C++
    calls them
    [fundamental types](https://en.cppreference.com/w/cpp/language/types). Since
    the Rust terminology is probably well understood by everybody, we use it
    here.
[^char32_t]: Unlike Rust `char`, `char16_t` and `char32_t` may contain invalid
    Unicode characters.
[^int128]: b/532400117. The reverse direction, Rust `i128` / `u128` to C++, is
    not yet supported (b/254094650).
[^char]: Note that Rust `c_char` and C++ `char` have different signedness in
    Google, or any other codebase with widespread use of unsigned `char` in x86.

    TODO(jeanpierreda): document this in more detail.
