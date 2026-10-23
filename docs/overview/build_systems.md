# Build System Integrations

Crubit works in two directions. Each direction has a build rule that you add
next to the library you want to call:

*   [`cpp_api_from_rust`](../rust/index.md) lets **C++ call Rust**.
*   [`rust_api_from_cpp`](../cpp/index.md) lets **Rust call C++**.

Here is where you can use each of them today:

| Build system                  | `cpp_api_from_rust` (C++ calling Rust)                                                                              | `rust_api_from_cpp` (Rust calling C++)  |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------ | :-------------------------------------- |
| [Bazel](bazel.md)             | [Supported](bazel.md#cpp_api_from_rust) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/bazel)) | [Supported](bazel.md#rust_api_from_cpp) |
| [CMake](cmake.md)             | [Supported](cmake.md) ([example](https://github.com/google/crubit/tree/main/examples/build_systems/cmake))                   | Not yet                                 |
| [Other](#other-build-systems) | Through the `cargo cpp_api_from_rust` subcommand                                                                    | Not yet                                 |

Crubit is used in production inside Google, and we are working towards the same
level of support for the open-source integrations; see [Usage outside of
Google](status.md#usage-outside-of-google) for the plan. Open-source CI builds
the Bazel and CMake examples on Linux x86_64 on every change to Crubit.

## Toolchain requirements {#toolchain-requirements}

Crubit is closely tied to particular compiler releases. This is what lets it
automatically generate bindings using idiomatic language features instead of
being limited to the C ABI.

### Rust

`cpp_api_from_rust` is built on the Rust compiler's internal APIs, so it must be
built by the same `rustc` that compiles the crates it generates bindings for,
and that toolchain must include the `rustc-dev` component. Crubit supports
recent nightly and stable Rust versions.

A daily test matrix checks that Crubit builds against the last 7 nightly
toolchains and the 2 most recent stable releases. The badges in the
[README](../index.md#test-matrix) show the current results, and
[`nightly.yaml`](https://github.com/google/crubit/blob/test-matrix/.github/workflows/nightly.yaml)
on the `test-matrix` branch is the workflow that produces them.

How to set up a compatible Rust toolchain depends on your build system and
channel:

*   **Bazel (nightly):** `crubit_toolchains.configure` in `MODULE.bazel`
    downloads a pinned nightly toolchain with `rustc-dev` for you; see
    [Bazel](bazel.md).

*   **Nightly with `rustup` (CMake or Cargo):** Install a recent nightly with
    the `rustc-dev` component:

    ```sh
    rustup toolchain install nightly --component rustc-dev
    ```

    In a Crubit checkout, `rust-toolchain.toml` selects a nightly toolchain with
    `rustc-dev` automatically.

*   **Stable with `rustup` (CMake or Cargo):** Add the `rustc-dev` component to
    one of the two most recent stable releases, and set `RUSTC_BOOTSTRAP=1` when
    building Crubit and the crates that use the generated bindings (because they
    rely on a few [unstable features](unstable_features.md)):

    ```sh
    rustup component add rustc-dev --toolchain stable
    export RUSTC_BOOTSTRAP=1
    ```

### C++

`cpp_api_from_rust` generates portable C++20. The examples and CI use Clang, but
the generated code doesn't depend on a particular compiler or Clang version.

`rust_api_from_cpp` parses C++ with Clang's libraries, so the Crubit binary is
built against a specific LLVM commit. The Bazel integration takes care of this
for you, and `MODULE.bazel` pins a commit that CI updates regularly. The LLVM
release that Bazel downloads for compiling C++ needs to be at least as new as
the LLVM that your Rust toolchain was built with; `crubit_toolchains` checks
this against the `rustc` found on your `PATH`, so a `rustc` must be on `PATH`
when Bazel fetches dependencies.

## Versions and releases

Crubit doesn't publish releases or semantic versions yet. Development happens on
`main`, and every day a GitHub Action moves the
[`latest`](https://github.com/google/crubit/tree/latest) tag to the newest
commit on `main` that passed CI. We recommend that you **pin a specific
commit**, such as the one `latest` points to, and upgrade on purpose. Binary
releases are planned; see [Usage outside of
Google](status.md#usage-outside-of-google).

## Other build systems

Crubit's tools are ordinary command-line programs, so you can run them from any
build system.

For C++ calling Rust, the `cargo cpp_api_from_rust` subcommand builds a Cargo
library package, runs Crubit on it, and writes a C++ header and a static library
that any C++ build system can consume. This is what the CMake integration uses.
It is built from `cargo/cc_bindings_from_rs/cargo-cpp_api_from_rust` in the
Crubit repository.

See [Cargo build of Crubit](cargo_build.md) for how to build the tools
themselves, and the Bazel rules for
[`cpp_api_from_rust`](https://github.com/google/crubit/tree/main/cc_bindings_from_rs/bazel_support)
and
[`rust_api_from_cpp`](https://github.com/google/crubit/tree/main/rs_bindings_from_cc/bazel_support)
for how they are invoked.
