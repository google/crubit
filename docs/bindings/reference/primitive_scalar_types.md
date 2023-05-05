# Bindings for primitive scalar types

This chapter describes how Crubit maps primitive scalar types like integers,
floating-point numbers, etc.

## Integer types

The following types are mapped bidirectionally:

C++         | Rust
----------- | -------
`int8_t`    | `i8`
`int16_t`   | `i16`
`int32_t`   | `i32`
`int64_t`   | `i64`
`intptr_t`  | `isize`
`uint8_t`   | `u8`
`uint16_t`  | `u16`
`uint32_t`  | `u32`
`uint64_t`  | `u64`
`uintptr_t` | `usize`

The following C++ types are mapped one-way into the corresponding Rust types:

C++         | Rust
----------- | -------
`ptrdiff_t` | `isize`
`size_t`    | `usize`
`char16_t`  | `u16`
`char32_t`  | `u32`
`wchar_t`   | `i32`

<!-- TODO(b/276790180): if we use `ffi::c_char` etc., document it here -->

The following C++ types are mapped in a platform-dependent way to the
corresponding Rust type of the same width and signedness:

C++                  | Rust
-------------------- | -----------------------
`char`               | `i8` or `u8`, or higher
`signed char`        | `i8` or higher
`unsigned char`      | `u8` or higher
`short`              | `i16` or higher
`unsigned short`     | `u16` or higher
`int`                | `i16` or higher
`unsigned int`       | `u16` or higher
`long`               | `i32` or higher
`unsigned long`      | `u32` or higher
`long long`          | `i64`
`unsigned long long` | `u64`

Bindings for 128-bit-wide integers are not supported at this point.

## Other scalar types

`rs_bindings_from_cc` and `cc_bindings_from_rs` also support the following
bi-directional type mapping:

C++               | Rust   | Notes
----------------- | ------ | -------------------------------------
`bool`            | `bool` |
`double`          | `f64`  |
`float`           | `f32`  |
`rs_std::rs_char` | `char` | See `crubit/support/rs_std/rs_char.h`
