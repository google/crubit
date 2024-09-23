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
bool IsGreater(int lhs, int rhs);
```

This function, if present in a header file which is processed by Crubit, becomes
callable from Rust as if it were defined as:

```rs
pub fn IsGreater(lhs: ffi::c_int, rhs: ffi::c_int) -> bool {...}
```

Note: There are some temporary restrictions on the API shape. For example,
functions that accept a type like `std::string` can't be called from Rust
directly via Crubit. These restrictions will be relaxed over time.

## Getting Started

Here are some resources for getting started with Crubit:

*   [Rust Bindings for C++ Libraries](https://github.com/google/crubit/tree/main/docs/cpp/)
    is a detailed walkthrough on how to use C++ from Rust using Crubit.

*   The [`examples/cpp/`](http://examples/cpp)
    directory has copy-pastable examples of calling C++ from Rust, together with
    snapshots of what the generated Rust interface looks like.

## Building Crubit

### Cargo

Prerequisites:
* Requires LLVM and Clang libraries to be built and installed.
  * They must be built with support for compression (zlib), which is the default
    build config.
* Requires Abseil libraries to be built and installed.
* Requires zlib (e.g. libz.so) to be available in the system include and lib
  paths.
* An up-to-date stable Rust toolchain.

Linux instructions:
```sh
# Paths for Crubit's cargo to use.
## This path contains clang/ and llvm/ dirs with their respective headers.
export CLANG_INCLUDE_PATH=/path/to/llvm/and/clang/headers
## This path contains libLLVM*.a and libclang*.a.
export CLANG_LIB_STATIC_PATH=/path/to/llvm/and/clang/libs
## This path contains absl/ dir with all the includes.
export ABSL_INCLUDE_DIR=/path/to/absl/include/dir
## This path contains libabsl_*
export ABSL_LIB_STATIC_PATH=/path/to/absl/libs

# Choice of compiler is optional.
export CC=/path/to/clang
export CXX=/path/to/clang++

# We must use `lld` linker via clang.
export PATH="$PATH:/dir/containing/lld"
export LD=ld.lld
# We use clang as the linker, it finds lld.
export RUSTFLAGS="$RUSTFLAGS -Clinker=/path/to/clang"
export LDFLAGS="$LDFLAGS -fuse-ld=lld"
export RUSTFLAGS="$RUSTFLAGS -Clink-arg=-fuse-ld=lld"

# Explicitly link the C++ std library for the C++ components.
export LDFLAGS="$LDFLAGS -lstdc++"
export RUSTFLAGS="$RUSTFLAGS -Clink-arg=-lstdc++"

# If you want to use a sysroot.
# SYSROOT_FLAG=--sysroot=$SYSROOT
# export CFLAGS="$CFLAGS $SYSROOT_FLAG"
# export CXXFLAGS="$CXXFLAGS $SYSROOT_FLAG"
# export LDFLAGS="$LDFLAGS $SYSROOT_FLAG"
# export RUSTFLAGS="$RUSTFLAGS -Clink-arg=$SYSROOT_FLAG"

cargo build --bin rs_bindings_from_cc
```

### Bazel

```sh
apt install clang lld bazel
git clone git@github.com:google/crubit.git
cd crubit
bazel build --linkopt=-fuse-ld=/usr/bin/ld.lld //rs_bindings_from_cc:rs_bindings_from_cc_impl
```

#### Using a prebuilt LLVM tree

```sh
git clone https://github.com/llvm/llvm-project
cd llvm-project
CC=clang CXX=clang++ cmake -S llvm -B build -DLLVM_ENABLE_PROJECTS='clang' -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=install
cmake --build build -j
# wait...
cmake --install build
cd ../crubit
LLVM_INSTALL_PATH=../llvm-project/install bazel build //rs_bindings_from_cc:rs_bindings_from_cc_impl
```
