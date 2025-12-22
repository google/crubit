<!-- <internal link> -->

# Cargo Build of Crubit

crubit.rs/overview/cargo_build

<!--*
# Document freshness: For more information, see <internal link>.
freshness: { owner: 'lukasza' reviewed: '2025-12-19' }
*-->

[TOC]

## cc_bindings_from_rs

`cc_bindings_from_rs` can be built by invoking `cargo` as follows:

```
$ cargo build --release --verbose --bin cc_bindings_from_rs \
    --manifest-path \
    $CRUBIT_GIT_REPO/cargo/cc_bindings_from_rs/cc_bindings_from_rs/Cargo.toml
```

### Troubleshooting

#### `rustc_driver*.rmeta` build dependency

If `cargo build ... --bin cc_bindings_from_rs` fails to build, then check
if the following problem has been reported as one of the first errors:

```
error[E0463]: can't find crate for `rustc_driver`
  --> cargo/cc_bindings_from_rs/run_compiler/../../../cc_bindings_from_rs/run_compiler.rs:10:1
   |
10 | extern crate rustc_driver;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
```

If you build Crubit using `cargo` and `rustc` that are managed by `rustup`,
then you can just run the suggested command to add the `rustc-dev` component:
`rustup component add rustc-dev`.

If you build Crubit using `cargo` and `rustc`
that are built and installed using `x.py install`,
then please ensure that `x.py`'s `config.toml` covers the `rustc-dev` component
(h/t @Nadrieril for [the PR](https://github.com/rust-lang/rust/pull/149655)
to teach `x.py install` about this component).
For example:

```
$ cat third_party/rust-src/config.toml
...
# With `extended = true`, `x.py install` will install rustc and the listed
# tools. This is the *only* way to install rust-analyzer-proc-macro-srv, which
# is necessary to use rust-analyzer.
extended = true
tools = [
    "cargo",
    "clippy",
    "rustc-dev",  # <= ASKS `./x.py install` TO COVER `rustc_driver*.rmeta`
    "rustfmt",
    "rust-analyzer",
    "rust-analyzer-proc-macro-srv",
    "src",
]
...
```

#### `librustc_driver*.so` runtime dependency

`cc_bindings_from_rs` depends at runtime on `librustc_driver*.so`.
If the runtime linker cannot find this library,
then launching `cc_bindings_from_rs` may fail with the following error:

```
$ third_party/rust-toolchain/bin/cc_bindings_from_rs --help
...
error while loading shared libraries: librustc_driver-871558eb5abca9d6.so:
cannot open shared object file: No such file or directory
```

`rustc` is able to find the `librustc_driver*.so` library,
because (on certain platforms) `x.py` sets a `RUNPATH` in the `rustc` binary:

```
$ readelf -d third_party/rust-toolchain/bin/rustc | grep RUNPATH
 0x000000000000001d (RUNPATH)            Library runpath: [$ORIGIN/../lib]
$ ls third_party/rust-toolchain/lib/librustc_driver-*.so
third_party/rust-toolchain/lib/librustc_driver-871558eb5abca9d6.so
```

If `cc_bindings_from_rs` is distributed/installed next to `rustc`, then
you can fix the linking error by setting an identical `RUNPATH` when
building `cc_bindings_from_rs`.
This can be done by asking `cargo`
to pass additional `rustc` command-line flags
(e.g. using `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS`
[environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)).
The following command-line flags should work
(they are based on
[`x.py`'s sources](https://github.com/rust-lang/rust/blob/b889870082dd0b0e3594bbfbebb4545d54710829/src/bootstrap/src/core/builder/cargo.rs#L285-L306)):

* Linux:
    - `-Clink-args=-Wl,-z,origin`
    - `-Clink-args=-Wl,-rpath,$ORIGIN/../lib`
* Mac:
    - `-Zosx-rpath-install-name`
    - `-Clink-args=-Wl,-rpath,@loader_path/../lib`
* Windows:
    - no extra command-line flags needed (TODO: verify this)

## rs_bindings_from_cc

Cargo build of `rs_bindings_from_cc` is not supported at this point.
TODO(b/379928127): Describe this build target once it is supported.
