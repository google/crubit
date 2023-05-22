# Crubit assumptions about `extern "C"` ABI of built-in Rust types

`cc_bindings_from_rs` makes certain assumptions about internal implementation
details of the Rust compiler. In particular, `cc_bindings_from_rs` assumes a
specific ABI of `extern “C”` functions that pass/return values of types that the
`improper_ctypes_definitions` warning complains about. Because of these
assumptions, the generated `..._cc_api_impl.rs` code disables the warning via
`#![allow(improper_ctypes_definitions)]`. These ABI assumptions are documented
below.

## Rust built-in `char` type

`extern “C”` thunks generated in `..._cc_api_impl.rs` can take `char` arguments
(and can return `char` values). (Note that this section talks about the Rust
`char` type which is different from the C++ `char` type.)

[Rust documentation says](https://rust-lang.github.io/unsafe-code-guidelines/layout/scalars.html#char)
that “Rust char is 32-bit wide” and
[that](https://doc.rust-lang.org/reference/type-layout.html)
`size_of::<char>() == 4` . Additionally,
[Rust documentation describes](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
some invalid bit patterns that may result in undefined behavior: “a value in a
`char` which is a surrogate or above `char::MAX`”.

Rust does *not* directly document the alignment of `char`, although it does
[say](https://doc.rust-lang.org/reference/type-layout.html) that “most
primitives are generally aligned to their size”. Furthermore Rust does *not*
guarantee a particular ABI (e.g. whether `char` value can be passed in a
general-purpose register VS in a vector register VS has to be passed by
pointer). `cc_bindings_from_rs` assumes that Rust `char` has the same alignment
and ABI as `uint32_t` (and therefore the same ABI as `rs_std::rs_char` from
`crubit/support/rs_std/rs_char.h`).

The assumptions are verified by assertions that verify the properties of the
target achitecture when `cc_bindings_from_rs` runs (`layout.align()`,
`layout.size()`, and `layout.abi()` assertions in `format_ty_for_cc` in
`cc_bindings_from_rs/bindings.rs`). Similar assertions are verified on C++ side
in `support/rs_std/rs_char_test.cc`. These assertions seem unlikely to fail, but
if they do, then hopefully `rs_char` can just be tweaked to wrap another of the
C++ integer types.

## Rust built-in `&[T]` slice reference type

In the *future* `extern “C”` thunks generated in `..._cc_api_impl.rs` may take
`&[i32]` and similar arguments (or return them).

[Rust documentation describes](https://rust-lang.github.io/unsafe-code-guidelines/layout/arrays-and-slices.html)
the layout of arrays and slices and
[also documents](https://doc.rust-lang.org/std/primitive.slice.html) that slice
references are “represented as a pointer and a length”.

Rust does *not* document the ABI of slice references (i.e. if the pointer comes
before or after the length in memory). `cc_bindings_from_rs` assumes that `&[T]`
has the same ABI as (future) `rs_std::slice<T>` - a C++ struct with 2 fields: a
`T*` pointer, and the `size_t` number of slice elements. TODO: Add runtime
assertions to `bindings.rs` to further verify these assumptions. TODO: Specify a
plan of action when the assertions fail.

`cc_bindings_from_rs` does *not* assume that `&[T]` and `rs_std::slice<T>` have
the same ABI as
[`std::span<T>`](https://en.cppreference.com/w/cpp/container/span) from C++ 20.
In particular, empty slices have a different representation in C++ and in Rust -
conversions implemented by `rs_std::slice<T>` will take care of using a null or
non-null pointer as appropriate.

## Rust built-in `&str` string reference

In the *future* `extern “C”` thunks generated in `..._cc_api_impl.rs` may take
`&str` and similar arguments (or return them).
[Rust documentation says](https://doc.rust-lang.org/std/primitive.str.html) that
“a &str is made up of two components: a pointer to some bytes, and a length”,
but no additional ABI guarantees are specified.

`cc_bindings_from_rs` assumes that `&str` has the same ABI as `&[u8]` (see the
previous section) with
[the additional requirement](https://doc.rust-lang.org/std/primitive.str.html)
that the contents of `[u8]` “are always valid UTF-8”. A future
`rs_std::str_slice` type will enforce the UTF-8 guarantees. TODO: Add runtime
assertions to `bindings.rs` to further verify these assumptions. TODO: Specify a
plan of action when the assertions fail.

`cc_bindings_from_rs` does *not* assume that `&str` and `rs_std::str_slice` have
the same ABI as
[`std::string_view`](https://en.cppreference.com/w/cpp/string/basic_string_view)
from C++ 17. In particular, references to empty string slices have a different
representation in C++ and in Rust - conversions implemented by
`rs_std::str_slice` will take care of using a null or non-null pointer as
appropriate.
