# Rust bindings for C++ fundamental types

Here we describe how Crubit maps the fundamental types like integers,
floating-point numbers, etc.

## Bidirectional map of types

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

## One-way map of C++ into Rust types

The C++ types below are mapped one-way into the corresponding Rust types.
("one-way" means that the type doesn't round-trip - for example `size_t` maps to
`usize`, but `usize` maps to `uintptr_t`.)

TODO(b/283258442): `::core::ffi::*` should eventually be a bidirectional mapping

| C++                  | Rust                       | Notes                    |
| -------------------- | -------------------------- | ------------------------ |
| `ptrdiff_t`          | `isize`                    |                          |
| `size_t`             | `usize`                    |                          |
| `char16_t`           | `u16`                      |                          |
| `char32_t`           | `u32`                      | Unlike `rs_std::rs_char` |
:                      :                            : above, `char32_t` may    :
:                      :                            : contain invalid Unicode  :
:                      :                            : characters               :
| `char`               | `u8` or `i8` depending on  | TODO(b/276790180): This  |
:                      : whether `char` is signed   : may eventually become    :
:                      : on the target platform     : `c_char`                 :
| `signed char`        | `::core::ffi::c_schar`     |                          |
| `unsigned char`      | `::core::ffi::c_uchar`     |                          |
| `short`              | `::core::ffi::c_short`     |                          |
| `unsigned short`     | `::core::ffi::c_ushort`    |                          |
| `int`                | `::core::ffi::c_int`       |                          |
| `unsigned int`       | `::core::ffi::c_uint`      |                          |
| `long`               | `::core::ffi::c_long`      |                          |
| `unsigned long`      | `::core::ffi::c_ulong`     |                          |
| `long long`          | `::core::ffi::c_longlong`  |                          |
| `unsigned long long` | `::core::ffi::c_ulonglong` |                          |

## Unsupported types

Bindings for the following types are not supported at this point:

-   `u128` and `i128` (b/254094650)
-   `wchar_t` (b/283268558)
