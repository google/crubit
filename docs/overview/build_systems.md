<!-- <internal link> -->

# Build System Integrations

crubit.rs/overview/build_systems

<!--*
# Document freshness: For more information, see <internal link>.
freshness: { owner: 'ethansmith' reviewed: '2026-05-13' }
*-->

[TOC]

## CMake

[cpp_api_from_rust](crubit.rs/rust/index) has support for building with
[CMake](https://cmake.org/), allowing C++ code built by CMake to call Rust code
built with Cargo through Crubit generated bindings. This support builds on
[Corrosion](https://github.com/corrosion-rs/corrosion), the premier marriage
between Rust and CMake. To set up a CMake project using Crubit you'll need two
pieces:

  * A CMake Project
  * A Cargo Project (embedded within the CMake project)

You can find an example setup project at
[`examples/build_systems/cmake`](https://github.com/google/crubit/tree/main/examples/build_systems/cmake).
We'll walk through what that project does
[below](crubit.rs/overview/build_systems#example-setup).

### Build Configuration

Crubit uses Cargo to build your Rust library. Modifications to your Cargo.toml
will be respected and reflected when the Crubit CMake targets are built. We only
require that your project provides a library package (specifically we need it to
provide an `.rlib` or `.rmeta` file).

On the CMake side, Corrosion provides a suite of options to tune the Cargo
build. See their [usage
docs](https://corrosion-rs.github.io/corrosion/usage.html) for details. These
are threaded through to the underlying cargo build as expected, with some
notable exceptions:

  * `CRATES` will not work. **We do not support Cargo Workspaces yet.**
  * `CRATE_TYPES` -- We always import a lib/rlib with Crubit
  * `OVERRIDE_CRATE_TYPE` -- We must use `--crate-type=lib`

The Corrosion-generated CMake target is an interface library containing the
generated header and a static library containing the Rust code and generated FFI
bindings.

### Example Setup

To setup Crubit with CMake, first we'll need a CMake project, see their [their
documentation](https://cmake.org/cmake/help/latest/guide/tutorial/Getting%20Started%20with%20CMake.html)
for help configuring one. At minimum, you'll need a directory (we'll call ours
`cmake`) with a `CMakeLists.txt` at its root containing:

```cmake
cmake_minimum_required(VERSION 3.22)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_C_COMPILER clang)
set(CMAKE_CXX_COMPILER clang++)

project(example_project
  VERSION 0.1.0
  LANGUAGES CXX)
```

From there we'll need a [Cargo
project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html). We can create
one using the shell command:

```sh
cmake/$ cargo new --lib rust_lib
```

Because we're using Crubit to call our Rust code, our cargo project must produce
a normal Rust library (and not a binary, static library, cdylib, etc.).

Our two projects are combined through Corrosion. In our `CMakeLists.txt`, we
fetch Corrosion and enable it for our Cargo project `rust_lib`:

```cmake
include(FetchContent)

FetchContent_Declare(
  Corrosion
  # NB: We temporarily require a fork of corrossion as we work to upstream our changes.
  GIT_REPOSITORY https://github.com/thunderseethe/corrosion
  GIT_TAG main
)

FetchContent_MakeAvailable(Corrosion)
```

We use [`Corrosion`](https://github.com/corrosion-rs/corrosion), a Cargo CMake
integration, to add our Cargo project to our CMake project:

```cmake
corrosion_experimental_crubit(rust_lib)
corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)
```

`rust_lib` is the name of our library package in our Cargo project, which
defaults to crate name. If we were to rename our package, we'd have to update
this to reflect the package name rather than the crate or directory name.
Corrosion takes a path to the Cargo.toml of our project and uses that to extract
the necessary metadata.

Under the hood, Corrosion generates a CMake library target named `rust_lib`,
after our Rust package. We can use that target to depend on Rust from our C++
code. If we have a CMake executable `main`, we can add a dependency on Rust by
adding the following to our `CMakeLists.txt`:

```cmake
add_executable(main main.cpp)
target_link_libraries(main PRIVATE rust_lib)
```

Crubit generates a C++ header for Rust library based on the library package
name, as well. In `main.cpp`, we can import our `crubit/rust_lib.h` header:

```
{{ #include ../../examples/build_systems/cmake/main.cpp }}
```
<!--  -->


With that we're calling Rust code from C++! Our CMake integration is subject to
the same [limitations](crubit.rs/overview/limits.md) as our other build
integrations.
