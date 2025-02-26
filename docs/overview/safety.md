# Memory Safety

This page attempts to document the soundness properties of Crubit itself, as a
layer on top of existing C++ APIs.

## C++ interfaces

**Crubit does not guarantee that its bindings are sound.** Nor could it.
Instead, it makes the following assumptions:

1.  Functions which are not marked `CRUBIT_UNSAFE` only require validity of
    their parameters for safety. Such functions will not require `unsafe`.

2.  Rust moves of a trivially relocatable (and replaceable) object produce a
    valid object with the same state. A Rust move of a trivially relocatable
    object does not cause UB in C++.

When these are not true of the C++ functions, then the corresponding Rust
functions will be unsound. This can be fixed by marking types as `CRUBIT_UNSAFE`
(when they logically entail certain preconditions for validity, such as
lifetime), or by marking individual functions as `CRUBIT_UNSAFE`.

## Build Environment

The C++/Rust interfaces that Crubit sees **must** be ABI-compatible with the
real C++/Rust interfaces. To ensure this, Crubit requires the following of the
build environment:

*   The wrapped libraries are built using the same version of the compiler that
    Crubit uses, with the same compilation settings (flags, environment
    variables, etc.). For example, an extra `-DSOME_DEFINE` or a new version of
    the compiler can completely change the ABI, or even the API, of a given
    library. If the compiler sees it, but Crubit does not, the generated
    bindings will be incorrect.
    *   In particular, Crubit's outputs (*e.g.* `..._cc_api.h` or
        `..._rs_api.rs`) are not stored in a source code repository, as they
        *will* go out of date. Crubit comes with Bazel integration support to
        invoke it at build time.
*   The ABI must be deterministic and reproducible (*i.e.* depend only on
    compilation settings, the compiler version, and the wrapped libraries
    themselves). For example, the
    [`-Zrandomize-layout`](https://github.com/rust-lang/compiler-team/issues/457)
    flag cannot be used without a fixed seed.
