# `ffi_11`: One-to-one FFI types

In `ffi_11`, if a type is distinct in C/C++, it is distinct in Rust.

For example, `char` is a distinct type from `signed char`, even on platforms
where `char` is signed, and so `ffi_11::c_char` is always a distinct type from
`ffi_11::c_schar`.

For more details, see
[the API documentation](https://docs.rs/ffi_11/latest/ffi_11/).

## Why a new `ffi` crate?

The `std::ffi` module has a few shortcomings:

1.  It does not match C or C++ on a one-to-one basis: it's possible to write out
    APIs in C++ which have no Rust equivalent, and cannot be called from Rust.
2.  It doesn't contain entries for every fundamental type: it is missing recent
    additions to C and C++, such as `nullptr_t`, (C++11, C23) or `char8_t`
    (C++20).
3.  Because types like `c_char` are just aliases for other, existing types, it
    presents a portability trap: even the *documentation* for functions
    accepting a `c_char` will instead say `i8` in public documentation
    ([example](https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_raw)),
    even though it's `u8` on other platforms. There are no compiler errors or
    lint warnings if you use `i8` with APIs that actually accept `c_char`.

Problem 3 can be solved with improvements to the documentation generator and
linters. These tools can use an approach like
[Crubit's `SugaredTy`](https://github.com/google/crubit/commit/6a813346b846acd0394187d3e58745b11d052a62)
to keep track of whether `i8` came from the `c_char` alias, even though it is
not inherently part of the type system. It doesn't need a new `ffi` module!

Problem 2 is hopefully only a matter of adding some new aliases/types to the
`ffi` module. It, also, doesn't need a new `ffi` module.

Problem 1, however, is fundamental to the design of `std::ffi`.

The standard library `ffi` module defines multiple aliases to the same type,
where in C++ they would be different types. For example, Rust has two byte types
(`i8`, `u8`), but C++ has at least four (`char`, `signed char`, `unsigned char`,
and `char8_t`). This makes it fundamentally impossible to express certain C++
APIs as Rust APIs using the `ffi` module. Consider the following C++ constructor
overload set:

```c
struct Foo {
  Foo(char) {std::cout << "char\n";}
  Foo(signed char) {std::cout << "signed char\n";}
  Foo(unsigned char) {std::cout << "unsigned char\n";}
};
```

If we wanted to map these constructors to a `From` impl, there is no way to
write out all three! We might try the following:

```c
impl From<ffi::c_char> for Foo {...}
impl From<ffi::c_schar> for Foo {...}
impl From<ffi::c_uchar> for Foo {...}
```

But on x86, `c_char` and `c_schar` are the same type, and so this is a
compilation error:

```rust
error[E0119]: conflicting implementations of trait `From<i8>` for type `Foo`
  --> src/lib.rs:12:1
   |
8  | impl From<ffi::c_char> for Foo {
   | ------------------------------ first implementation here
...
12 | impl From<ffi::c_schar> for Foo {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Foo`

For more information about this error, try `rustc --explain E0119`.
```

This problem is not limited to bytes. Rust also has two 32-bit integer types.
C++ has 4 on Linux, and 5 on Windows. And so on, with varying exact numbers, for
all the sized types.

And this problem is not limited to obscure overload sets. If we want to support
templates, then what is to be done about `std::vector<c_char>` vs
`std::vector<c_schar>`? They cannot be the same type in Rust without UB, and
yet, since `c_char` and `c_schar` may be the same type, we end up in a bind:

```c++
// C++:
std::vector<char> Foo();
std::vector<signed char> Bar();
```

```rust
// Rust
pub fn Foo() -> std::vector<c_char>;
pub fn Bar() -> std::vector<  ??  >;
```

Ideally, we would like a way to express distinct types in C++ as distinct types
in Rust, for the purpose of distinguishing them at compile time in trait lookup
and templates/generics. We want `From<c_char>` to be different from
`From<c_schar>`, and we want `std::vector<c_char>` to be a different type to
`std::vector<c_schar>`, because they are different in C++, and losing the
distinction in Rust would means not every C++ API is callable.

To achieve this, we created an alternate take on the `ffi` module: `ffi_11`.

## Example

The `ffi_11` crate looks substantially identical to `ffi` from a user point of
view, and allows restating C++ APIs in Rust using its interop types. The
following C++ API and Rust API are equivalent, using `ffi_11`:

```c++
// C++
long long Foo(signed char, char);
```

```rust
// Rust
pub fn Foo(_: ffi11::c_schar, _: ffi11::c_char) -> ffi11::c_longlong;

// NOT EQUIVALENT TO `pub fn Foo(_: i8, _: i8) -> i64;` -- not on any platform.
```

Unlike `ffi` from the standard library, `c_schar` and `c_char` are guaranteed to
be different types.

## Why is it called `ffi_11`?

The name is meant to allude to three things:

*   `ffi`, but one-to-one (and onto).
*   `ffi`, but it also adds C++11 (and above) support. Many things that are
    C++11-aware is more modern – this is also reminiscent of, for instance,
    [`pybind11`](https://github.com/pybind/pybind11).
*   And, of course... it goes to 11!
