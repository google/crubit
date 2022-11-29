# cc_bindings_from_rs

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.


## Invoking the tool

Most `rustc` cmdline parameters should be supported (e.g. `--crate-type`).

The following example should work in the current dev environment:

<!-- BEGIN GOOGLE-INTERNAL -->
If you're a Googler, set `<path_of_rustfmt_executable>` to
 `"$(pwd)/../../unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt"`.
<!-- END GOOGLE-INTERNAL -->

```
$ echo > $HOME/scratch/test.rs "
pub extern \"C\" fn public_function() {
    private_function()
}
fn private_function() {}
"

$ cd crubit/cc_bindings_from_rs
$ bazel run :cc_bindings_from_rs_legacy_toolchain_runner -- \
    --h-out=$HOME/scratch/test.h \
    --rs-out=$HOME/scratch/test_impl.rs \
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

## Contributing

See
[rs_bindings_from_cc/README.md](../rs_bindings_from_cc/README.md#contributing).
