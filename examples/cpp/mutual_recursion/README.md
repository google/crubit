# Replacing one of two mutually-recursive C++ functions with Rust

This directory shows how to rewrite a single function of a `cc_library` in Rust
when that function and another C++ function in the same library call each other.

`parity.h` declares two functions, `parity::IsEven` and `parity::IsOdd`, each of
which calls the other:

*   `IsEven` is still implemented in C++, in `parity.cc`.
*   `IsOdd` is now implemented in Rust, as `is_odd::is_odd` in `is_odd.rs`,
    which calls `IsEven` through Crubit's Rust bindings for `parity.h`.
*   `parity.cc` keeps a one-line C++ definition of `parity::IsOdd` that calls
    `is_odd::is_odd` through Crubit's C++ API for the Rust crate
    (`cpp_api_from_rust`, `is_odd.h`). The C++ API and ABI of `parity::IsOdd`
    don't change, and neither do its callers.

## Breaking the dependency cycle

`:parity` depends on `:is_odd` (through `:is_odd_cc`), because `parity.cc` calls
`is_odd::is_odd`. `:is_odd` can't depend on `:parity` in return, because Bazel
doesn't allow dependency cycles. Instead, `:is_odd` gets its bindings for
`IsEven` from `:parity_decls`, a `cc_library` with the same header (`parity.h`)
but no `srcs`:

```
:parity ──> :is_odd_cc ──> :is_odd ──> :parity_decls_rust ──> :parity_decls
   │                                                               ▲
   └───────────────────────────────────────────────────────────────┘
```

The definition of `IsEven` still comes from `:parity`, so every binary that uses
`:is_odd` must also depend on `:parity` (see `main_rs` in the `BUILD` file). C++
binaries depend on `:parity` anyway.

## `alwayslink = True`

The functions still refer to each other at link time, even though the Bazel
targets don't. Bazel puts each library on the link line before the libraries it
depends on, so `:parity` comes before `:is_odd`. The linker only pulls a
library's object files into the link when an earlier file refers to one of their
symbols. If nothing has pulled in `parity.cc` by the time the linker reaches
`:is_odd`, for example in a Rust binary that only calls `is_odd::is_odd`, then
`:is_odd`'s reference to `IsEven` points backwards. lld resolves backward
references (traditional Unix linkers don't), but with `--warn-backrefs` (which
many toolchains enable, to catch link-order bugs) they are an error:

```
ld: error: backward reference detected: _ZN6parity6IsEvenEi in
.../libis_odd-<hash>.a(...) refers to .../_objs/parity/parity.pic.o
```

Setting `alwayslink = True` on `:parity` makes the linker always include
`parity.cc`, so `IsEven` is already defined when the linker reaches `:is_odd`.
No linker flags are needed.

## Alternatives

*   If none of the C++ code in the library needs to call the Rust function, no
    cycle exists: put the Rust function in a separate `rust_library`, and call
    it from C++ through `cpp_api_from_rust`.
*   To move a whole library to Rust at once, see `examples/cpp/carcinize`.
