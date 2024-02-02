# Crubit: C++/Rust Bidirectional Interop Tool

[![Build status](https://badge.buildkite.com/7a57a14e68aa3a0ab70972cbf2a35fd79d342ba152fee4a5b4.svg)](https://buildkite.com/bazel/crubit)

NOTE: Crubit currently expects deep integration with the build system, and is
difficult to deploy to environments dissimilar to Google's monorepo. We do not
have our tooling set up to accept external contributions at this time.

Crubit is a bidirectional bindings generator for C++ and Rust, with the goal of
integrating the C++ and Rust ecosystems. With Crubit, Rust can be used anywhere
C++ can be, directly calling and being called by C++. Crubit does not require
wrapping C++ or Rust libraries in "FFI-friendly", simplified APIs. Any C++
interface can be called or implemented by Rust code.

**Current status:** Crubit is aiming for an initial stable "MVP" version,
comparable to `bindgen` and `cbindgen`. This will support basic datatypes like
integers and pointers, (rust-movable) structs/unions/enums, and `extern "C"`
functions.

<!-- TODO(b/276366603): Link to reference doc which specifies *exactly* what is
supported. -->

## Example

Consider the following C++ function:

```c++
extern "C" bool IsAbsPath(std::string_view path);
```

This function, if present in a header file which is processed by Crubit, becomes
callable from Rust as if it were defined as:

```rs
pub fn IsAbsPath(path: std::string_view) -> bool {...}
```

There are some temporary restrictions on the API shape. For example, if `path`
were a `std::string`, or a non-`extern "C"` function, it would not be callable
from Rust directly via Crubit. (For example, `std::string` is not rust-movable.)
These restrictions will be relaxed over time.

For actual copy-pastable examples on getting started with Crubit, see the
[`examples/`](http://examples) subdirectory.
`examples/cpp` includes code which actually calls C++ via Crubit, and a snapshot
of what the generated interface looks like.

<!-- TODO(b/276366603): Link to a codelab and reference documentation.-->

## Building Crubit

```
$ apt install clang lld bazel
$ git clone git@github.com:google/crubit.git
$ cd crubit
$ bazel build --linkopt=-fuse-ld=/usr/bin/ld.lld //rs_bindings_from_cc:rs_bindings_from_cc_impl
```

### Using a prebuilt LLVM tree

```
$ git clone https://github.com/llvm/llvm-project
$ cd llvm-project
$ CC=clang CXX=clang++ cmake -S llvm -B build -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=install
$ cmake --build build -j
$ # wait...
$ cmake --install build
$ cd ../crubit
$ LLVM_INSTALL_PATH=../llvm-project/install bazel build //rs_bindings_from_cc:rs_bindings_from_cc_impl
```
