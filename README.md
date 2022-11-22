# Crubit: C++/Rust Bidirectional Interop Tool

[![Build status](https://badge.buildkite.com/7a57a14e68aa3a0ab70972cbf2a35fd79d342ba152fee4a5b4.svg)](https://buildkite.com/bazel/crubit)

Extremely experimental interop tooling for C++ and Rust.

Please don't use, this is an experiment and we don't yet know where will it take
us. There will be breaking changes without warning. Unfortunately, we can't take contributions at this point.

## Build

There are two supported ways to build Crubit.

* **Option 1: Local LLVM installation**. This option is recommended for most use cases. It involves building a local copy of LLVM code which will enable one to easily stay in sync with the latest version.

* **Option 2: Basel provided LLVM installation**. This option is recommended for developers that need to track an older version of LLVM.

### Install system dependencies

#### Install Bazelisk

Install Bazelisk following [these instructions](https://github.com/bazelbuild/bazelisk/blob/master/README.md).

#### Install system packages

```
$ apt install clang lld
```

#### Build local LLVM installation (**Option 1** only)

```
$ git clone https://github.com/llvm/llvm-project
$ cd llvm-project
$ CC=clang CXX=clang++ cmake -S llvm -B build -DLLVM_ENABLE_PROJECTS='clang;lld' -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=install
$ cmake --build build -j
$ # wait...
$ cmake --install build
$ cd ..
```

### Build Crubit

#### Build with local LLVM installation (**Option 1** only)

```
$ export LLVM_INSTALL_PATH=$PWD/llvm-project/install
$ export PATH="$LLVM_INSTALL_PATH/bin:$PATH"
$ cd crubit
$ bazel build --linkopt=-fuse-ld=$LLVM_INSTALL_PATH/bin/ld.lld //rs_bindings_from_cc:rs_bindings_from_cc_impl
```

#### Build with Basel provided LLVM installation (**Option 2** only)

```
$ apt install clang lld bazel
$ git clone git@github.com:google/crubit.git
$ cd crubit
$ bazel build --linkopt=-fuse-ld=/usr/bin/ld.lld //rs_bindings_from_cc:rs_bindings_from_cc_impl
```
