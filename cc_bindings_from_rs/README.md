# cc_bindings_from_rs

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.

## Invoking the tool on the command line

Most `rustc` cmdline parameters should be supported (e.g. `--crate-type`).

The following example should work in the current dev environment:

```
$ echo > $HOME/scratch/test.rs "
pub extern \"C\" fn public_function() {
    private_function()
}
fn private_function() {}
"

$ cd crubit/cc_bindings_from_rs
$ bazel run :cc_bindings_from_rs -- \
    --h-out=$HOME/scratch/test.h \
    --rs-out=$HOME/scratch/test_impl.rs \
    --clang-format-exe-path=<path_of_clang_format_executable> -- \
    --rustfmt-exe-path=<path_of_rustfmt_executable> -- \
    $HOME/scratch/test.rs \
    --crate-type=lib \
    --codegen=panic=abort

$ cat $HOME/scratch/test.h
// Automatically @generated C++ bindings for the following Rust crate:
// test

#pragma once

namespace test {
extern "C" void public_function();
}
```

## Invoking the tool through bazel aspect

You can inspect the generated bindings files by building the aspect:

```sh
bazel build --aspects  //cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl%cc_bindings_from_rust_aspect --output_group=out //some/rust:library
```

## Contributing

See
[rs_bindings_from_cc/README.md](../rs_bindings_from_cc/README.md#contributing).
