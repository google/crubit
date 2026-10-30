# Cargo Build of Crubit

This page explains how to build Crubit's own tools with Cargo. To use Crubit in
your project, start with [Build System Integrations](build_systems.md); the
Bazel and CMake integrations build the tools for you.

`cc_bindings_from_rs` uses the Rust compiler's internal APIs (`rustc_private`),
so it requires a toolchain with the `rustc-dev` component and must be built by
the same `rustc` that compiles the crates it generates bindings for. See
[Toolchain requirements](build_systems.md#toolchain-requirements) for supported
toolchains and setup instructions.

## cc_bindings_from_rs

In a Crubit checkout, `rustup` automatically selects the nightly toolchain from
`rust-toolchain.toml`, and you can build `cc_bindings_from_rs` with Cargo:

```sh
git clone https://github.com/google/crubit.git
cd crubit
cargo build --release -p cc_bindings_from_rs
```

Find more details on configuring the target directory and troubleshooting below.

If you are using a stable Rust toolchain, set the environment variable
`RUSTC_BOOTSTRAP=1`:

```
$ RUSTC_BOOTSTRAP=1 cargo build # further args as above
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

#### Unstable features on stable

Crubit uses unstable Rust features that are only available in nightly builds. We
rely on setting [the `RUSTC_BOOTSTRAP` environment
variable](https://doc.rust-lang.org/beta/unstable-book/compiler-environment-variables/RUSTC_BOOTSTRAP.html)
to `1` to tell a stable Rust compiler to act like a nightly compiler.

If you attempt to build `cc_bindings_from_rs` with a stable Rust toolchain and
do not set this variable, the build will fail with error messages like:

```
error[E0554]: `#![feature]` may not be used on the stable release channel
```

## rs_bindings_from_cc

`rs_bindings_from_cc` is a Rust binary that depends on C++ libraries (Abseil,
LLVM/Clang LibTooling, and Protobuf). To build it with Cargo in an open-source
checkout, you first need to build and export these C++ dependencies.

### Building with C++ dependencies from Bazel

This is how Crubit's CI checks that the tool builds with Cargo. If you can use
Bazel, prefer the [full Bazel build](../cpp/building.md#bazel) instead.

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

### Building with CMake

The CMake project in `cmake/rs_bindings_from_cc` builds `rs_bindings_from_cc`.
It finds or builds the C++ dependencies and then runs Cargo. For each
dependency, it uses one that's already installed if it's suitable, and
otherwise gets one itself:

*   **LLVM/Clang**: uses the install you pass with `-DCRUBIT_LLVM_ROOT=<path>`,
    or else an LLVM 23.1.3 that `find_package(LLVM)` finds (for example through
    `CMAKE_PREFIX_PATH` or `LLVM_DIR`). Either one must contain
    `bin/llvm-config`, the headers and the static Clang and LLVM libraries,
    and `llvm-ar` must be from the same LLVM version. Otherwise, it downloads
    the official LLVM 23.1.3 release for your platform. On platforms that LLVM
    doesn't publish releases for (such as x86_64 macOS), you must provide LLVM
    explicitly.
*   **Abseil and Protobuf**: uses the installed packages that
    `find_package(protobuf)` finds, if they are static libraries and the
    Protobuf version matches the `protobuf` crate dependency in `Cargo.toml`.
    Otherwise, it builds both from source, at the versions that the Bazel build
    uses. Most system packages are shared libraries or a different Protobuf
    version, so expect it to build them.

Every download is checked against a pinned SHA-256 hash. If you change
`CRUBIT_LLVM_VERSION` or `CRUBIT_LLVM_URL`, set `CRUBIT_LLVM_URL_HASH` too;
otherwise the build warns that the download isn't verified.

You need CMake 3.22 or later, Ninja, a C++20 compiler, and `rustup`. On Linux and
macOS, run from the root of the checkout:

```sh
$ cmake -S cmake/rs_bindings_from_cc -B cmake_build -G Ninja
$ cmake --build cmake_build
$ ./cmake_build/bin/rs_bindings_from_cc --help
```

The build also writes `cmake_build/crubit-cargo-env.sh`, which sets the environment
variables described [below](#custom-build-systems-chromium-android-or-other-projects).
After one CMake build, you can run Cargo yourself:

```sh
$ source cmake_build/crubit-cargo-env.sh
$ cargo build --release -p rs_bindings_from_cc
```

Useful cache variables:

*   `CRUBIT_LLVM_ROOT`: an existing LLVM install to use.
*   `CRUBIT_LLVM_URL` and `CRUBIT_LLVM_URL_HASH`: a different LLVM archive to
    download, and its hash as `SHA256=<hex>`.
*   `CRUBIT_LLVM_DEV_DATE`: the date (`YYYYMMDD`) of the LLVM commit. It is
    detected for LLVM 23 releases (23.1.0 and later); set it for any other
    LLVM, including builds of LLVM's main branch.
*   `CRUBIT_FIND_DEPENDENCIES`: whether to look for installed Abseil and
    Protobuf packages (default `ON`). Set it to `OFF` to always build them.
*   `CRUBIT_CARGO_PROFILE`: the Cargo profile to build with (default
    `release`).

To use the tool from another CMake project, add this project with
`add_subdirectory()`. Because it's outside that project's source tree, also
give it a build directory:

```cmake
add_subdirectory(<crubit>/cmake/rs_bindings_from_cc crubit_rs_bindings_from_cc)
```

It defines the imported executable target `crubit::rs_bindings_from_cc`. Use `$<TARGET_FILE:crubit::rs_bindings_from_cc>`
in custom commands and list the target in their `DEPENDS`, so that the tool is
built first. `cmake --install` installs the binary to `bin/`.

Cargo uses a copy of `Cargo.lock` in the build directory (through
`CARGO_RESOLVER_LOCKFILE_PATH`), because the checked-in `Cargo.lock` also lists
crates that only Bazel uses, which Cargo would remove. The copy keeps the pinned
versions, and the checked-in file stays untouched.

When the LLVM install includes `clang` and `lld`, the build uses them to
compile C code and to link.

> [!NOTE]
> LLVM releases ship their static libraries as LTO bitcode, which only a linker
> of the same LLVM version can read.


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

        *   `CXXFLAGS`: Extra C++ compiler flags. You must include
            `-DCRUBIT_LLVM_DEV_DATE=<YYYYMMDD>` in addition to standard flags
            like `-stdlib=libc++`. For example:

            ```sh
            LLVM_DATE=$(python3 cargo/build/get_llvm_commit_date.py <llvm_commit_hash>)
            export CXXFLAGS="-stdlib=libc++ -DCRUBIT_LLVM_DEV_DATE=${LLVM_DATE}"
            ```

        *   `RUSTFLAGS`: Rust compiler flags for linking (e.g. `-C linker=...`,
            `-C link-arg=-stdlib=libc++`, `-C link-arg=-lc++`, `-C
            link-arg=-lzstd`).

3.  **Build `rs_bindings_from_cc` using Cargo**:

    ```sh
    $ cargo build --locked -p rs_bindings_from_cc
    ```
