# Panics and Exceptions in C++/Rust FFI bindings

SUMMARY: Crubit currently requires `-fno-exceptions`, and converts unwinding
panics into aborts.

## Unwinding and Aborting

> "Alright so I'm panicking, what else is there to do?" \
> -- "The Hitchhiker’s Guide to the Galaxy" by Douglas Adams

Rust and C++ both support a form of unwinding exception/panic, where ordinary
control flow is terminated, and instead control proceeds up the stack, calling
destructors along the way, until it is caught, converted to a termination, or
completely unwinds the stack (which also leads to termination).

This has a performance cost (in that data must be managed soundly, destroyed if
appropriate, in case of unwinding), and a code complexity cost (in that unsafe
code must handle unexpected control flow edges soundly), so implementations of
both languages allow for unwinding to be disabled, in favor of immediate process
termination:

*   Rust:
    [`-Cpanic=abort`](https://doc.rust-lang.org/rustc/codegen-options/index.html#panic)
*   C++
    [`-fno-exceptions`](https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fexceptions)

Crubit, and any FFI, will work if unwinding is disabled for both sides of the
FFI boundary. But if it is enabled for one or both sides, and an exception or
panic unwinds past an FFI boundary, we need special support to ensure that the
behavior is defined.

## Supported Configurations

> "Who said anything about panicking?" snapped Arthur. "This is still just the
> culture shock. You wait till I've settled down into the situation and found my
> bearings. Then I'll start panicking." \
> -- "The Hitchhiker’s Guide to the Galaxy" by Douglas Adams

**Rust:** Rust: Crubit can create bindings for libraries built with either
`-Cpanic=abort` or `-Cpanic=unwind`.

If a panic unwinds past a Crubit FFI boundary, the process will terminate
(on rustc nightly[^terminate_requirements]), with
the **sole** exception of `extern "C-unwind"` functions. If you define an
`extern "C-unwind"` function, you must ensure that it is only called by C++ code
which enables exceptions. This responsibility is left to the caller.

**C++:** Crubit can create bindings for libraries built with `-fno-exceptions`.
We do not generate `extern "C-unwind"` interfaces which could propagate a C++
exception, and the behavior is undefined if an exception propagates past a
Crubit FFI boundary. (See b/200067087 for catching this at compile time.)

<!-- TODO(b/200067087): fail when exceptions are enabled, document above. -->

[^terminate_requirements]: We use the behavior of
    https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding
    to cause a crash. However, this is not yet
    incorporated into a stable Rust release, and requires
    nightly.

<!-- TODO(b/254049425): remove above note once crash reaches stable. -->
