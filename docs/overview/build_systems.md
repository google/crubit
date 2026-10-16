# Build System Integrations

Crubit works in two directions. Each direction has a build rule that you add
next to the library you want to call:

*   [`cpp_api_from_rust`](../rust/index.md) lets **C++ call Rust**.
*   [`rust_api_from_cpp`](../cpp/index.md) lets **Rust call C++**.

Here is where you can use each of them today:

Build system          | `cpp_api_from_rust` (C++ calling Rust)                                   | `rust_api_from_cpp` (Rust calling C++)
:-------------------- | :----------------------------------------------------------------------- | :-------------------------------------
[Bazel](bazel.md)     | [Supported](bazel.md#cpp_api_from_rust) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/bazel)) | [Supported](bazel.md#rust_api_from_cpp)
[CMake](cmake.md)     | [Supported](cmake.md) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/cmake)) | Not yet
[Other](#other-build-systems) | Through the `cargo cpp_api_from_rust` subcommand                    | Not yet

Open-source CI builds the Bazel and CMake examples on Linux x86_64 on every
change to Crubit. Crubit is used in production inside Google, and we are
working towards the same level of support for the open-source integrations;
see [Usage outside of Google](status.md#usage-outside-of-google) for the plan.

## Toolchain requirements {#toolchain-requirements}

Crubit is closely tied to particular compiler releases. That is what lets it
integrate tightly with the underlying language primitives, rather than being
limited to the C ABI.

### Rust

`cpp_api_from_rust` is built on the Rust compiler's internal APIs
(`rustc_private`), so it must be built by the same `rustc` that compiles the
crates it generates bindings for, and that toolchain must include the
`rustc-dev` component. Both nightly and stable toolchains work:

*   **Nightly:** `rustup component add rustc-dev`. Crubit's
    `rust-toolchain.toml` selects a nightly toolchain with `rustc-dev`
    automatically, and the Bazel integration downloads one for you.
*   **Stable:** `rustup component add rustc-dev` works on stable too. Set
    `RUSTC_BOOTSTRAP=1` when building Crubit and the crates that use the
    generated bindings, because they rely on a few
    [unstable features](unstable_features.md).

A daily test matrix checks that Crubit builds against the last 7 nightly
toolchains and the 2 most recent stable releases. The badges in the
[README](../index.md#test-matrix) show the current results, and
[`nightly.yaml`](https://github.com/google/crubit/blob/test-matrix/.github/workflows/nightly.yaml)
on the `test-matrix` branch is the workflow that produces them.

### C++

`cpp_api_from_rust` generates portable C++20. The examples and CI use Clang,
but the generated code doesn't depend on a particular compiler or Clang
version.

`rust_api_from_cpp` parses C++ with Clang's libraries, so the Crubit binary is
built against a specific LLVM commit. The Bazel integration takes care of this
for you, and `MODULE.bazel` pins a commit that CI updates regularly. The LLVM
release that Bazel downloads for compiling C++ needs to be at least as new as
the LLVM that your Rust toolchain was built with; `crubit_toolchains` checks
this for you.

## Versions and releases

Crubit doesn't publish releases or semantic versions yet. Development happens
on `main`, and every day a GitHub Action moves the
[`latest`](https://github.com/google/crubit/tree/latest) tag to the newest
commit on `main` that passed CI. We recommend that you **pin a specific
commit**, such as the one `latest` points to, and upgrade on purpose. Binary
releases are planned; see
[Usage outside of Google](status.md#usage-outside-of-google).

## Other build systems

Crubit's tools are ordinary command-line programs, so you can run them from
any build system:

*   For C++ calling Rust, the `cargo cpp_api_from_rust` subcommand builds a
    Cargo library package, runs Crubit on it, and writes a C++ header and a
    static library that any C++ build system can consume. This is what the
    CMake integration uses. It is built from
    `cargo/cc_bindings_from_rs/cargo-cpp_api_from_rust` in the Crubit
    repository.
*   Chromium integrates both directions into GN with templates in its own
    build files. Crubit's `BUILD.gn` and `crubit.gni` declare the runtime
    support libraries that the generated code depends on.

See [Cargo build of Crubit](cargo_build.md) for how to build the tools
themselves, and the Bazel rules for
[`cpp_api_from_rust`](https://github.com/google/crubit/tree/main/cc_bindings_from_rs/bazel_support)
and
[`rust_api_from_cpp`](https://github.com/google/crubit/tree/main/rs_bindings_from_cc/bazel_support)
for how they are invoked.
