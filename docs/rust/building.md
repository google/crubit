<!-- Part of the Crubit project, under the Apache License v2.0 with LLVM -->
<!-- Exceptions. See /LICENSE for license information. -->
<!-- SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception -->

# Building

This page is about calling **Rust from C++** with `cpp_api_from_rust`. If you
want to call C++ from Rust, see [Rust bindings for C++
libraries](../cpp/building.md) instead.

`cpp_api_from_rust` is available in these build systems:

Build system                   | How                                                           | Example
:----------------------------- | :------------------------------------------------------------ | :------
[Bazel](../overview/bazel.md)  | The `cpp_api_from_rust` rule from the `rules_crubit` module   | [`examples/build_systems/bazel`](https://github.com/google/crubit/tree/main/examples/build_systems/bazel)
[CMake](../overview/cmake.md)  | `corrosion_experimental_crubit()` from [Corrosion](https://github.com/corrosion-rs/corrosion) | [`examples/build_systems/cmake`](https://github.com/google/crubit/tree/main/examples/build_systems/cmake)

Each build system page above walks through the setup. Other build systems can
use the `cargo cpp_api_from_rust` subcommand, which writes a C++ header and a
static library for a Cargo package; see
[Build System Integrations](../overview/build_systems.md#other-build-systems).

## Quick start

The Crubit repository includes ready-to-run CMake and Bazel example projects
where a C++ `main` binary calls into a Rust library through bindings generated
by `cpp_api_from_rust`.

To run the examples on Linux x86_64, you need:

*   **CMake:** CMake 3.22 or later, Ninja, Clang (which the example's
    `CMakeLists.txt` uses to compile the C++ binary), and `rustup` (which
    installs the Rust toolchain from Crubit's `rust-toolchain.toml`).
*   **Bazel:** `bazelisk` (which downloads the Rust and Clang toolchains for
    you), plus a `rustc` on your `PATH`, which `crubit_toolchains` uses to check
    LLVM version compatibility.

{{#tabs}}

{{#tab name="CMake"}}

```sh
git clone https://github.com/google/crubit.git
cd crubit/examples/build_systems/cmake
cmake -B build -S . -G Ninja
cmake --build build
./build/main
```

{{#endtab}}

{{#tab name="Bazel"}}

```sh
git clone https://github.com/google/crubit.git
cd crubit/examples/build_systems/bazel
bazelisk run //:main
```

{{#endtab}}

{{#endtabs}}

## Toolchains

`cpp_api_from_rust` works with nightly Rust toolchains and with the two most
recent stable releases, as long as the `rustc-dev` component is installed. It
generates portable C++20 that doesn't depend on a particular C++ compiler. See
[Toolchain requirements](../overview/build_systems.md#toolchain-requirements)
for details.

## Building `cc_bindings_from_rs` yourself

The Bazel and CMake integrations build Crubit for you. If you're integrating
Crubit into another build system, see
[Cargo build of Crubit](../overview/cargo_build.md#cc_bindings_from_rs) for how
to build the `cc_bindings_from_rs` binary from source.
