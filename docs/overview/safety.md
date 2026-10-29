# Memory Safety

Crubit is designed so that mixing C++ and Rust does not give up the memory
safety that Rust provides. Compared to hand-written FFI, Crubit bindings are
safer in several ways:

*   **Bindings come straight from your compilers.** Crubit generates bindings
    from the same C++ and Rust compilers, flags, and sources that build your
    code, so the bindings always match the real declarations, layout, and ABI.
    You no longer have to wrangle hand-written `extern "C"` declarations or
    `#[repr(C)]` mirror structs that silently drift out of sync as an API
    changes; Crubit regenerates them on every build.
*   **`unsafe` only where it matters.** Only functions with real safety
    preconditions, such as those taking raw pointers, are `unsafe` in Rust,
    rather than every call that crosses the language boundary. The `unsafe`
    blocks in your code are the ones worth reviewing.
*   **Lifetime annotations for C++.** Crubit understands lifetimes on C++
    references, inferred by Rust's elision rules or written explicitly in the
    C++ declaration, and turns them into Rust references that the borrow checker
    enforces. See [References and
    lifetimes](../cpp/references_and_lifetimes.md).

## What Crubit assumes about C++ APIs

Crubit cannot prove that arbitrary C++ code is sound. Instead, the safe Rust
signatures it generates rest on two assumptions about the C++ API:

1.  A function has no safety preconditions beyond the validity of its
    parameters.
2.  A type that Crubit treats as
    [Rust-movable](../types/non_rust_movable/intro_short.md) can be moved by
    Rust without breaking any C++ invariants.

Where these do not hold, mark the function or type `CRUBIT_UNSAFE` (see
[Customizing](../cpp/customizing.md)), and the Rust bindings will require
`unsafe`.

## Build environment

Crubit's guarantees depend on the bindings seeing exactly the C++ and Rust
interfaces that are actually compiled:

*   Wrapped libraries must be built with the same compiler version and
    compilation settings (flags, environment variables, etc.) that Crubit uses.
    An extra `-DSOME_DEFINE` or a different compiler version can change a
    library's ABI or API. **For this reason, Crubit's outputs are generated at
    build time and must not be checked into source control**, where they would
    go out of date.
*   The ABI must be deterministic and reproducible, depending only on the
    compilation settings, the compiler version, and the wrapped libraries. For
    example,
    [`-Zrandomize-layout`](https://github.com/rust-lang/compiler-team/issues/457)
    cannot be used without a fixed seed.

Crubit's build integrations ([Bazel](bazel.md) and [CMake](cmake.md)) handle
this for you: they run Crubit as part of the build, with the same toolchain and
flags as the code it binds.
