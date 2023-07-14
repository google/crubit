# Crubit: C++/Rust Bidirectional Interop Tool

[![Build status](https://badge.buildkite.com/7a57a14e68aa3a0ab70972cbf2a35fd79d342ba152fee4a5b4.svg)](https://buildkite.com/bazel/crubit)

Crubit is an experimental bidirectional bindings generator for C++ and Rust.

Please don't use, this is an experiment and we don't yet know where will it take
us. There will be breaking changes without warning.  Unfortunately, we can't
take contributions at this point.

Crubit allows for C++ code and Rust code to call each other without manually
wrapping the APIs in an FFI-friendly interop layer. For example, a C++ function
like this:

```c++
bool IsAbsPath(std::string_view path);
```

... becomes callable from Rust as if it were defined as:

```rs
pub fn IsAbsPath(path: std::string_view) -> bool {...}
```

Crubit automatically generates ABI-compatible bindings for structs (which can be
passed both by value and by reference), functions, and methods, for a large
variety of types. (Trivial types, nontrivial types, templated types, etc.)

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
