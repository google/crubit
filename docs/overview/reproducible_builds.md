# Reproducible builds

Crubit can emit bindings for Rust or C++ libraries so that they can be called
from the other language, but these bindings require strict ABI compatibility. To
accomplish this, Crubit uses `clang` and `rustc` as libraries to discover the
public APIs of the wrapped libraries, and the ABI that they expose when built
(e.g. their function calling conventions, and the memory layout of their
structs).

To ensure that the generated bindings are ABI-compatible with the built
libraries, Crubit needs to see exactly what the real compiler sees:

*   Build the wrapped libraries using the same version of the compiler that
    Crubit uses, with the same compilation settings (flags, environment
    variables, etc.). For example, an extra `-DSOME_DEFINE` or a new version of
    the compiler can completely change the ABI, or even the API, of a given
    library. If the compiler sees it, but Crubit does not, the generated
    bindings will be incorrect.
*   Do not store Crubit outputs (*e.g.* `..._cc_api.h` or `..._rs_api.rs`) in a
    source code repository, as they *will* go out of date. Crubit comes with
    Bazel integration support to invoke it at build time.
*   The ABI must be deterministic and reproducible (*i.e.* depend only on
    compilation settings, the compiler version, and the wrapped libraries
    themselves). For example, the
    [`-Zrandomize-layout`](https://github.com/rust-lang/compiler-team/issues/457)
    flag cannot be used without a fixed seed.
