# CMake

CMake projects can call Rust from C++ with
[`cpp_api_from_rust`](../rust/index.md), through
[Corrosion](https://github.com/corrosion-rs/corrosion), the standard way to
combine CMake and Cargo. Corrosion builds your crate with Cargo, runs Crubit on
it, and gives you a CMake target that contains the generated header and a static
library with the Rust code.

Crubit's Corrosion support is in
[a fork of Corrosion](https://github.com/thunderseethe/corrosion) while the
[upstream pull request](https://github.com/corrosion-rs/corrosion/pull/666) is
reviewed. <!-- TODO(b/574505417): Point at upstream Corrosion once merged. -->

Rust bindings for C++ libraries (`rust_api_from_cpp`) aren't available in CMake
yet. <!-- TODO(b/539627008) -->

There are two example projects, and CI builds both on every change to Crubit:

*   [`examples/build_systems/cmake`](https://github.com/google/crubit/tree/main/examples/build_systems/cmake):
    the minimal setup walked through below.
*   [`examples/build_systems/cmake_demo`](https://github.com/google/crubit/tree/main/examples/build_systems/cmake_demo):
    a slightly larger demo that uses several crates.io dependencies.

To run the minimal example, you need CMake 3.22 or later, Ninja, Clang, and
`rustup` (it installs the Rust toolchain from Crubit's `rust-toolchain.toml`):

```sh
git clone https://github.com/google/crubit.git
cd crubit/examples/build_systems/cmake
cmake -B build -S . -G Ninja
cmake --build build
./build/main
```

## Build configuration

Crubit uses Cargo to build your Rust library. Changes to your `Cargo.toml` are
respected when the Crubit CMake targets are built. Your project needs to
provide a library package, because Crubit needs it to produce an `.rlib` or
`.rmeta` file.

On the CMake side, Corrosion has many options for tuning the Cargo build. See
the [Corrosion usage
docs](https://corrosion-rs.github.io/corrosion/usage.html) for details. Most of
these options are passed through to the underlying Cargo build, with a few
exceptions:

*   `CRATES` doesn't work, because `cargo cpp_api_from_rust` generates bindings
    for the single library package in the root manifest. Cargo workspaces
    aren't supported yet. <!-- TODO(b/574506047) -->
*   `CRATE_TYPES` has no effect, because Crubit always imports a lib/rlib.
*   `OVERRIDE_CRATE_TYPE` has no effect, because Crubit must use
    `--crate-type=lib`.

The CMake target that Corrosion creates is an interface library. It contains
the generated header and a static library with the Rust code and the generated
FFI bindings.

## Example setup

First, create a CMake project. See the [CMake
documentation](https://cmake.org/cmake/help/latest/guide/tutorial/Getting%20Started%20with%20CMake.html)
for help. At a minimum, you need a directory (ours is named `cmake`) with a
`CMakeLists.txt` at its root that contains:

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

Next, create a [Cargo
project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) with this
shell command:

```sh
cmake/$ cargo new --lib rust_lib
```

C++ calls this code through Crubit, so the Cargo project must produce a normal
Rust library. A binary, static library, or cdylib won't work.

Corrosion connects the two projects. In `CMakeLists.txt`, fetch Crubit and
Corrosion, then turn on Crubit for the `rust_lib` package:

```cmake
include(FetchContent)

FetchContent_Declare(
  crubit
  GIT_REPOSITORY https://github.com/google/crubit
  GIT_TAG <commit SHA>  # If omitted, Corrosion fetches Crubit's latest version.
)

FetchContent_Declare(
  Corrosion
  # NB: We temporarily require a fork of Corrosion while we upstream our changes.
  GIT_REPOSITORY https://github.com/thunderseethe/corrosion
  GIT_TAG main
)

FetchContent_MakeAvailable(Corrosion)

corrosion_experimental_crubit(rust_lib)
corrosion_import_crate(MANIFEST_PATH rust_lib/Cargo.toml)
```

`rust_lib` is the name of the library package in the Cargo project. By default,
it's the same as the crate name. If you rename the package, update this line to
use the package name, not the crate or directory name. Corrosion reads the
project's `Cargo.toml` to get the metadata it needs.

Corrosion creates a CMake library target named after the Rust package. C++ code
uses that target to depend on the Rust code. For example, to make a CMake
executable `main` depend on it, add the following to `CMakeLists.txt`:

```cmake
add_executable(main main.cpp)
target_link_libraries(main PRIVATE rust_lib)
```

Crubit names the generated C++ header after the library package. In
`main.cpp`, include `crubit/rust_lib.h`:

```
{{ #include ../../examples/build_systems/cmake/main.cpp }}
```
<!--  -->


Now C++ can call the Rust code. See
[C++ bindings for Rust libraries](../rust/index.md) for what the generated API
looks like. The CMake integration has the same [limitations](limits.md) as
Crubit's other build integrations.
