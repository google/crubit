# Rust bindings for C++ fundamental types

Crubit maps most C++ fundamental types to the direct Rust equivalent. For
example, `int32_t` becomes `i32`, `int` becomes `ffi::c_int`, `double` becomes
`f64`, and so on.

The exceptions are the currently-unsupported types `nullptr_t`, `char8_t`,
`wchar_t`, and `(u)int128_t`.

## Bidirectional map of C++ types

The following map is bidirectional. If you call a C++ interface from Rust using
Crubit, then `int32_t` in C++ becomes `i32` in Rust. Vice versa, if you call a
Rust interface from C++ using Crubit, `i32` in Rust becomes `int32_t` in C++.

C++                  | Rust
-------------------- | -------------------------------------------------------
`void`               | `()` as a return type, `::core::ffi::c_void` otherwise.
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
`char`               | `::core::ffi::c_char` [^char]
`signed char`        | `::core::ffi::c_schar`
`unsigned char`      | `::core::ffi::c_uchar`
`short`              | `::core::ffi::c_short`
`unsigned short`     | `::core::ffi::c_ushort`
`int`                | `::core::ffi::c_int`
`unsigned int`       | `::core::ffi::c_uint`
`long`               | `::core::ffi::c_long`
`unsigned long`      | `::core::ffi::c_ulong`
`long long`          | `::core::ffi::c_longlong`
`unsigned long long` | `::core::ffi::c_ulonglong`

## One-way map of C++ into Rust types

The C++ types below are mapped one-way into the corresponding Rust types. For
example `size_t` maps to `usize`, but `usize` maps to `uintptr_t`.

C++         | Rust
----------- | -----------------
`ptrdiff_t` | `isize`
`size_t`    | `usize`
`char16_t`  | `u16`
`char32_t`  | `u32` [^char32_t]

## Unsupported types

Bindings for the following types are not supported at this point:

*   `nullptr_t` and `char8_t` have not yet been implemented.
*   b/283268558: `wchar_t` is currently unsupported, for portability reasons.
*   b/254094650: `int128_t` is currently unsupported, because it does not yet
    have a decided ABI.

[^char32_t]: Unlike Rust `char`, `char16_t` and `char32_t` may contain invalid
    Unicode characters.
[^char]: Note that Rust `c_char` and C++ `char` have different signedness in
    Google, or any other codebase with widespread use of unsigned `char` in
    x86.

    TODO(jeanpierreda): document this in more detail.
