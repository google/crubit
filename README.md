# Crubit: C++/Rust Bidirectional Interop Tool

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