# cc_bindings_from_rs

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.


## Invoking as a standalone executable

Most `rustc` cmdline parameters should be supported (e.g. `--crate-type`).

The following example should work in the current dev environment:

```
$ cd crubit/cc_bindings_from_rs
$ bazel build :cc_bindings_from_rs
...
$ CC_BINDINGS_FROM_RS_BINARY=$(find `bazel info bazel-bin` -name cc_bindings_from_rs -type f)
$ export LD_LIBRARY_PATH=$PWD/../../unsupported_toolchains/rust/toolchains/nightly/lib/rustlib/x86_64-unknown-linux-gnu/lib

$ echo > test.rs "
pub fn public_function() {
    private_function()
}
fn private_function() {}
"

$ $CC_BINDINGS_FROM_RS_BINARY --h-out=test.h -- test.rs --crate-type=lib \
    --sysroot $(pwd)/../../unsupported_toolchains/rust/toolchains/nightly

$ cat test.h
// Automatically @generated C++ bindings for the following Rust crate:
// test

// Error while generating bindings for `public_function` defined at [...]/scratch/test.rs:1:1: 1:25: Nothing works yet!
```

## Contributing

See
[rs_bindings_from_cc/README.md](../rs_bindings_from_cc/README.md#contributing).
