# Crubit: C++/Rust Bidirectional Interop Tool

[![Build status](https://badge.buildkite.com/7a57a14e68aa3a0ab70972cbf2a35fd79d342ba152fee4a5b4.svg)](https://buildkite.com/bazel/crubit)

NOTE: Crubit currently expects deep integration with the build system, and is
difficult to deploy to environments dissimilar to Google's monorepo. We do not
have our tooling set up to accept external contributions at this time.

Crubit is a bidirectional bindings generator for C++ and Rust, with the goal of
integrating the C++ and Rust ecosystems.

## Status

Support for calling FFI-friendly C++ from Rust is in progress.

Support for calling Rust from C++ will arrive in 2024H2.

## Example

Consider the following C++ function:

```c++
extern "C" bool IsGreater(int lhs, int rhs);
```

This function, if present in a header file which is processed by Crubit, becomes
callable from Rust as if it were defined as:

```rs
pub fn IsGreater(lhs: ffi::c_int, rhs: ffi::c_int) -> bool {...}
```

Note: There are some temporary restrictions on the API shape. For example,
functions that are not `extern "C"`, or that accept a type like `std::string`,
can't be called from Rust directly via Crubit. These restrictions will be
relaxed over time.

## Getting Started

Here are some resources for getting started with Crubit:

*   [Rust Bindings for C++ Libraries](https://github.com/google/crubit/tree/main/docs/cpp/)
    is a detailed walkthrough on how to use C++ from Rust using Crubit.

*   The [`examples/cpp/`](http://examples/cpp)
    directory has copy-pastable examples of calling C++ from Rust, together with
    snapshots of what the generated Rust interface looks like.

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
