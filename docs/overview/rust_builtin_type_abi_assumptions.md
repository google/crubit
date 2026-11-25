# Crubit assumptions about `extern "C"` ABI of built-in Rust types

`cc_bindings_from_rs` makes certain assumptions about internal implementation
details of the Rust compiler. In particular, `cc_bindings_from_rs` assumes a
specific ABI of `extern “C”` functions that pass/return values of types that the
`improper_ctypes_definitions` warning complains about. Because of these
assumptions, the generated `..._cc_api_impl.rs` code disables the warning via
`#![allow(improper_ctypes_definitions)]`. These ABI assumptions are documented
below.

## Rust built-in `char` type

(Note that this section talks about the Rust `char` type, which is different
from the C++ `char` type.)

[The Rust standard library documentation for
`char`](https://doc.rust-lang.org/std/primitive.char.html#validity-and-layout)
guarantees that "`char` is guaranteed to have the same size, alignment, and
function call ABI as `u32` on all platforms". Additionally, [the Rust Reference
describes](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
the invalid bit patterns that result in undefined behavior: "a value in a `char`
which is a surrogate or above `char::MAX`".

Crubit maps Rust `char` to `rs::char_` (defined in `crubit/support/rs/char.h`),
a standard-layout C++ class wrapping a single `std::uint32_t` field and
enforcing the Unicode scalar value invariant. Because Rust `char` is guaranteed
to have the same layout as `u32` (which matches `std::uint32_t`), `rs::char_`
has the same size, alignment, and memory representation as Rust `char` (verified
on the C++ side by `static_assert`s in `support/rs_std/char_test.cc`).

Note that although Rust `char` has the same function call ABI as `u32`, a C++
class wrapping `std::uint32_t` (`rs::char_`) does *not* have the same function
call ABI as a scalar `std::uint32_t` on all platforms (for example, MSVC x64 and
32-bit System V i386 return `rs::char_` indirectly via a hidden pointer rather
than in a register). Crubit therefore passes and returns `char` / `rs::char_`
indirectly by pointer in `extern "C"` thunks rather than assuming by-value ABI
compatibility between `char` and `rs::char_`. `rustc`'s `improper_ctypes` lint
still warns on `char` in `extern "C"` declarations (including behind pointers),
so generated bindings keep `#![allow(improper_ctypes)]` /
`#![allow(improper_ctypes_definitions)]`.

## Rust built-in `[T]` slice type

`extern “C”` thunks generated in `..._cc_api_impl.rs` can take `&[T]`,
`&mut [T]`, `*const [T]` and similar arguments (or return them).

[Rust documentation
describes](https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html)
the layout of references and pointers to arrays and slices and [also
documents](https://doc.rust-lang.org/std/primitive.slice.html) that slice
references are “represented as a pointer and a length”.

Rust does *not* document the ABI of slice references (i.e. if the pointer comes
before or after the length in memory). `cc_bindings_from_rs` assumes that `&[T]`
has the same layout as `rs::SliceRef<T>` - a C++ struct with 2 fields: a `T*`
pointer, and the `size_t` number of slice elements. `check_slice_layout` inside
`crubit/cc_bindings_from_rs/generate_bindings/format_type.rs` verifies the
assumptions on the Rust slice each time a reference or a pointer to a slice is
processed by `format_type.rs`. Similar assertions are verified on C++ side in
`support/rs_std/slice_ref_test.cc`. The response to the unlikely failure of
these assertions can be changing the types from which `rs::SliceRef` is built to
match the Rust side again.

`cc_bindings_from_rs` does *not* assume that `&[T]` and `rs::SliceRef<T>` have
the same ABI as
[`std::span<T>`](https://en.cppreference.com/w/cpp/container/span) from C++ 20.
In particular, empty slices have a different representation in C++ and in Rust -
conversions implemented by `rs::SliceRef<T>` will take care of using a null or
non-null pointer as appropriate.

## Rust built-in `&str` string reference

`extern “C”` thunks generated in `..._cc_api_impl.rs` can take `&str` and
similar arguments (or return them). [Rust documentation
says](https://doc.rust-lang.org/std/primitive.str.html) that “a \&str is made up
of two components: a pointer to some bytes, and a length”, but no additional ABI
guarantees are specified.

`cc_bindings_from_rs` assumes that `&str` has the same ABI as `&[u8]` (see the
previous section) with [the additional
requirement](https://doc.rust-lang.org/std/primitive.str.html) that the contents
of `[u8]` “are always valid UTF-8”. The `rs::StrRef` type enforces the UTF-8
guarantees. Runtime assertions (`check_slice_layout`) further verify these ABI
layout assumptions.

`cc_bindings_from_rs` does *not* assume that `&str` and `rs::StrRef` have the
same ABI as
[`std::string_view`](https://en.cppreference.com/w/cpp/string/basic_string_view)
from C++ 17. In particular, references to empty string slices have a different
representation in C++ and in Rust - conversions implemented by `rs::StrRef` will
take care of using a null or non-null pointer as appropriate.
