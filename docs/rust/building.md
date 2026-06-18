<!-- Part of the Crubit project, under the Apache License v2.0 with LLVM -->
<!-- Exceptions. See /LICENSE for license information. -->
<!-- SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception -->

# Building

To use Rust libraries from C++, you need to integrate Crubit into your build
system.

## CMake Support

Currently, Crubit supports integration with CMake via
[Corrosion](https://github.com/corrosion-rs/corrosion). This allows C++ targets
in CMake to depend on Rust library targets built with Cargo, with Crubit
automatically generating the bindings.

For a detailed walkthrough on how to set up CMake integration, see the
[Build System Integrations](../overview/build_systems.md#cmake) page.

### Example Demo

You can find a complete, working example project showing how to set up CMake
with Crubit in
[`examples/build_systems/cmake`](https://github.com/google/crubit/tree/main/examples/build_systems/cmake).

## Other Build Systems

### Bazel

We intend to add support for Bazel soon.
