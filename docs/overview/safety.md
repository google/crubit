# Memory Safety

Crubit is designed so that mixing C++ and Rust does not give up the memory
safety that Rust provides. Compared to hand-written FFI, Crubit bindings are
safer in several ways:

*   **Types and signatures are checked by the compiler, not by hand.** Crubit
    reads the C++ API from the C++ compiler (and the Rust API from the Rust
    compiler), so the generated bindings always match the real declarations.
    There are no hand-written `extern "C"` declarations or `#[repr(C)]` mirror
    structs that can silently drift out of sync as an API changes, which is the
    most common source of undefined behavior in hand-written FFI.
*   **ABI compatibility is guaranteed by construction.** Bindings are generated
    at build time, with the same compiler and flags used to build the wrapped
    library, so layout, calling convention, and ABI always agree. See
    [Build environment](#build-environment) below.
*   **Safe by default, `unsafe` where it matters.** Functions that are safe to
    call with valid arguments get safe Rust signatures. Functions that take raw
    pointers, or that are annotated `CRUBIT_UNSAFE`, are `unsafe fn`s, so the
    places that need review are the places marked `unsafe`.
*   **References carry lifetimes.** C++ references become Rust references, with
    lifetimes taken from Rust's elision rules or from explicit lifetime
    annotations on the C++ declaration, so the Rust borrow checker enforces
    them. See [References and lifetimes](../cpp/references_and_lifetimes.md).
*   **Non-trivial C++ objects stay valid.** Types that C++ cannot safely
    relocate are exposed as non-Rust-movable, pinned types rather than as plain
    Rust values, so Rust cannot move them out from under C++. See
    [Movable types](../rust/movable_types.md).

## What Crubit assumes about C++ APIs

Crubit cannot prove that arbitrary C++ code is sound. Instead, the safe Rust
signatures it generates rest on two assumptions about the C++ API:

1.  A function that is not marked `CRUBIT_UNSAFE` has no safety preconditions
    beyond the validity of its parameters.
2.  A Rust move of a type that Crubit treats as Rust-movable (trivially
    relocatable or `[[clang::trivial_abi]]`) produces a valid object with the
    same state.

Where these do not hold, mark the function or type `CRUBIT_UNSAFE` (see
[Customizing](../cpp/customizing.md)); the Rust bindings then require `unsafe`
and the caller takes on the C++ preconditions, just as they would for any other
`unsafe` API.

## Build environment

Crubit's guarantees depend on the bindings seeing exactly the C++ and Rust
interfaces that are actually compiled:

*   Wrapped libraries must be built with the same compiler version and
    compilation settings (flags, environment variables, etc.) that Crubit uses.
    An extra `-DSOME_DEFINE` or a different compiler version can change a
    library's ABI or API. For this reason, Crubit's outputs are generated at
    build time and are not checked into source control, where they would go out
    of date.
*   The ABI must be deterministic and reproducible, depending only on the
    compilation settings, the compiler version, and the wrapped libraries. For
    example,
    [`-Zrandomize-layout`](https://github.com/rust-lang/compiler-team/issues/457)
    cannot be used without a fixed seed.

Crubit's build integrations ([Bazel](bazel.md), [CMake](cmake.md), and
[Cargo](cargo_build.md)) take care of this.
