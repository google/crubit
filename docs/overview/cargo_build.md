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
    $CRUBIT_ROOT/cargo/cc_bindings_from_rs/cc_bindings_from_rs/Cargo.toml
```

This will work from a git checkout of the public project or from Google's
internal mirror of the repo. If you plan to iterate on a cargo build, it is
recommended to `cd third_party/crubit` so you can build/test without specifying
the manifest path explicitly:

```
$ cargo build -p cc_bindings_from_rs
```

## Target Directory

A default Cargo build will generate a `target/` directory adjacent to the
Cargo.toml that will clutter your VCS client with intermediate build files. You
can avoid this by setting your target directory somewhere outside your VCS. A
`config.toml` can set your target directory to a non-standard location:

```toml
[build]
# You can also use something like `~/.cargo/target` if you want to keep the intermediaries around
target-dir = "/tmp/target/"
```

If you want your `config.toml` to apply to all your projects, place it at
`~/.cargo/config.toml`. See [Cargo's configuration
documentation](https://doc.rust-lang.org/cargo/reference/config.html) for the
other locations you can place a `config.toml` and their scope.

### Troubleshooting

#### `rustc_driver*.rmeta` build dependency

If `cargo build ... --bin cc_bindings_from_rs` fails to build, then check if the
following problem has been reported as one of the first errors:

```
error[E0463]: can't find crate for `rustc_driver`
  --> cargo/cc_bindings_from_rs/run_compiler/../../../cc_bindings_from_rs/run_compiler.rs:10:1
   |
10 | extern crate rustc_driver;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
   |
   = help: maybe you need to install the missing components with: `rustup component add rust-src rustc-dev llvm-tools-preview`
```

If you build Crubit using `cargo` and `rustc` that are managed by `rustup`, then
you can just run the suggested command to add the `rustc-dev` component: `rustup
component add rustc-dev`.

If you build Crubit using `cargo` and `rustc` that are built and installed using
`x.py install`, then please ensure that `x.py`'s `config.toml` covers the
`rustc-dev` component (h/t @Nadrieril for [the
PR](https://github.com/rust-lang/rust/pull/149655) to teach `x.py install` about
this component). For example:

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

`cc_bindings_from_rs` depends at runtime on `librustc_driver*.so`. If the
runtime linker cannot find this library, then launching `cc_bindings_from_rs`
may fail with the following error:

```
$ third_party/rust-toolchain/bin/cc_bindings_from_rs --help
...
error while loading shared libraries: librustc_driver-871558eb5abca9d6.so:
cannot open shared object file: No such file or directory
```

`rustc` is able to find the `librustc_driver*.so` library, because (on certain
platforms) `x.py` sets a `RUNPATH` in the `rustc` binary:

```
$ readelf -d third_party/rust-toolchain/bin/rustc | grep RUNPATH
 0x000000000000001d (RUNPATH)            Library runpath: [$ORIGIN/../lib]
$ ls third_party/rust-toolchain/lib/librustc_driver-*.so
third_party/rust-toolchain/lib/librustc_driver-871558eb5abca9d6.so
```

If `cc_bindings_from_rs` is distributed/installed next to `rustc`, then you can
fix the linking error by setting an identical `RUNPATH` when building
`cc_bindings_from_rs`. This can be done by asking `cargo` to pass additional
`rustc` command-line flags (e.g. using `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS`
[environment
variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)).
The following command-line flags should work (they are based on [`x.py`'s
sources](https://github.com/rust-lang/rust/blob/b889870082dd0b0e3594bbfbebb4545d54710829/src/bootstrap/src/core/builder/cargo.rs#L285-L306)):

*   Linux:
    -   `-Clink-args=-Wl,-z,origin`
    -   `-Clink-args=-Wl,-rpath,$ORIGIN/../lib`
*   Mac:
    -   `-Zosx-rpath-install-name`
    -   `-Clink-args=-Wl,-rpath,@loader_path/../lib`
*   Windows:
    -   no extra command-line flags needed (TODO: verify this)

## rs_bindings_from_cc

`rs_bindings_from_cc` is a Rust binary that depends on C++ libraries (Abseil,
LLVM/Clang LibTooling, and Protobuf). To build it with Cargo in an open-source
checkout, you first need to build and export these C++ dependencies.

### Building with Bazel Dependencies (OSS Quickstart)

Crubit provides a helper script `cargo/build/setup_bazel_env.py` that discovers
prebuilt C++ dependencies in Bazel's output tree, merges them into monolithic
static archives, pre-generates Protobuf headers, and exports the necessary
environment variables.

1.  **Prebuild C++ dependencies using `bazelisk`**:

    ```sh
    $ bazelisk build \
        --remote_download_outputs=all \
        rs_bindings_from_cc:rs_bindings_from_cc_main \
        @protobuf//:protoc
    ```

2.  **Discover and package Bazel outputs**:

    ```sh
    $ python3 cargo/build/setup_bazel_env.py
    ```

3.  **Configure your shell environment**:

    ```sh
    $ source target/bazel_outputs/bazel-env.sh
    ```

4.  **Build `rs_bindings_from_cc` using Cargo**:

    ```sh
    $ cargo build --locked -p rs_bindings_from_cc
    ```

### Custom Build Systems (Chromium, Android, or Other Projects)

When integrating `rs_bindings_from_cc` into custom build systems (such as
Chromium, Android, or other standalone environments), you must prepare prebuilt
C++ dependencies and pre-generate C++ Protobuf headers before invoking `cargo`:

1.  **Pre-generate C++ Protobuf Headers**: To avoid build ordering issues and
    race conditions during parallel Cargo builds, C++ Protobuf headers
    (`rs_bindings_from_cc/ir.pb.h` and
    `rs_bindings_from_cc/generate_bindings/generate_bindings.pb.h`) must be
    generated prior to invoking `cargo build` (see
    [cargo_build_protobuf.md](cargo_build_protobuf.md) for architectural
    details). Crubit provides a helper script that invokes `protoc` to
    pre-generate these headers:

    ```sh
    $ python3 cargo/build/generate_proto_headers.py \
        --protoc=<path/to/protoc> \
        --out_dir=<path/to/generated_headers>
    ```

2.  **Configure Environment Variables**: Before invoking `cargo`, configure the
    following environment variables:

    *   **Abseil**:
        *   `ABSL_INCLUDE_PATH`: Comma-separated list of Abseil header include
            roots (containing `absl/...`).
        *   `ABSL_LIB_STATIC_PATH`: Comma-separated list of directories
            containing Abseil static archives (`.a` on Unix, `.lib` on Windows).
    *   **LLVM / Clang**:
        *   `CLANG_INCLUDE_PATH`: Comma-separated list of include directories
            for LLVM and Clang headers (`clang/...`, `llvm/...`, and
            tablegen-generated headers).
        *   `CLANG_LIB_STATIC_PATH`: Comma-separated list of directories
            containing Clang/LLVM static archives.
    *   **Protobuf**:
        *   `PROTOBUF_INCLUDE_PATH`: Comma-separated list of include directories
            containing Google Protobuf headers (`google/protobuf/...`,
            `utf8_range`), as well as Crubit's pre-generated Protobuf headers
            directory from step 1.
        *   `PROTOBUF_LIB_STATIC_PATH`: Directory containing the Protobuf static
            archive (`libprotobuf.a` or `protobuf.lib`).
        *   `PROTOC`: Absolute path to the `protoc` compiler executable. This is
            required by `protobuf-codegen` during the Cargo build to generate
            Rust Protobuf bindings for `ir_rust_proto` and
            `generate_bindings_rust_proto`.
    *   **Toolchain & Linker**:
        *   `CC` / `CXX`: C and C++ compilers (e.g. Clang/Clang++).
        *   `CXXFLAGS`: Extra C++ compiler flags (e.g. `-stdlib=libc++`).
        *   `RUSTFLAGS`: Rust compiler flags for linking (e.g. `-C linker=...`,
            `-C link-arg=-stdlib=libc++`, `-C link-arg=-lc++`, `-C
            link-arg=-lzstd`).

3.  **Build `rs_bindings_from_cc` using Cargo**:

    ```sh
    $ cargo build --locked -p rs_bindings_from_cc
    ```
