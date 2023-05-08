# C++ nullability analysis

Annotating C++ API boundaries with nullability information can improve their
Rust bindings (e.g. binding non-null pointers as `T&` rather than `Option<T&>`).

This directory has tools for C++ codebases that use such annotations:

- **Nullability inference** suggests annotations to add to APIs, by analyzing
  the code that implements and uses them.

- **Nullability verification** verifies that annotated APIs are used and
  implemented safely, e.g. checking nullable pointers before dereferencing them.
  This is a local analysis suitable for use in a clang-tidy check.

They use Clang, its [dataflow framework][], and its [nullability annotations][].

## Style

This directory mostly uses [LLVM-style][] C++, rather than Google-style C++ used
in the rest of `crubit/`. The goal is to make it easy to upstream into
clang-tidy once mature.

Specifically:

- We follow the LLVM coding standards, with the exceptions listed here.
- We use absl `CHECK()` rather than `assert()`.
  (This finds bugs more reliably, and is trivial to migrate later.)
- We otherwise avoid relying on absl, using llvm's Support libraries instead.
- We write `// TODO` instead of `// FIXME`.

This list isn't set in stone: we can choose to diverge further from LLVM style,
if it's worth more cost of upstreaming later.

[dataflow framework]: <https://github.com/llvm/llvm-project/tree/main/clang/include/clang/Analysis/FlowSensitive>
[nullability annotations]: <https://clang.llvm.org/docs/AttributeReference.html#nullability-attributes>
[LLVM-style]: <https://llvm.org/docs/CodingStandards.html>
