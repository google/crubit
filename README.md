# Crubit: C++/Rust Bidirectional Interop Tool

[![Build status](https://badge.buildkite.com/7a57a14e68aa3a0ab70972cbf2a35fd79d342ba152fee4a5b4.svg)](https://buildkite.com/bazel/crubit)

Extremely experimental interop tooling for C++ and Rust.

Please don't use, this is an experiment and we don't yet know where will it take
us. There will be breaking changes without warning. Unfortunately, we can't take
contributions at this point.

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
