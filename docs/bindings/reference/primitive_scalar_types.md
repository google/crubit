# Bindings for primitive scalar types

Here we describe how Crubit maps primitive scalar types like integers,
floating-point numbers, etc.

## Bidirectional mapping of types

The following types are mapped bidirectionally:

C++               | Rust    | Notes
----------------- | ------- | -------------------------------------
`int8_t`          | `i8`    |
`int16_t`         | `i16`   |
`int32_t`         | `i32`   |
`int64_t`         | `i64`   |
`intptr_t`        | `isize` |
`uint8_t`         | `u8`    |
`uint16_t`        | `u16`   |
`uint32_t`        | `u32`   |
`uint64_t`        | `u64`   |
`uintptr_t`       | `usize` |
`bool`            | `bool`  |
`double`          | `f64`   |
`float`           | `f32`   |
`rs_std::rs_char` | `char`  | See `crubit/support/rs_std/rs_char.h`

## One-way mapping of C++ into Rust types

The C++ types below are mapped one-way into the corresponding Rust types.
("one-way" means that the mapping doesn't round-trip - for example `size_t` maps
to `usize`, but `usize` maps to `uintptr_t`.)

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

## Unsupported types

Bindings for 128-bit-wide integers are not supported at this point.
