# Bazel

Crubit is a Bazel module named `rules_crubit`. It provides both directions of
bindings:

*   [`cpp_api_from_rust`](../rust/index.md) generates a C++ library from a
    `rust_library`. Load it from `@rules_crubit//bazel:cpp_api_from_rust.bzl`.
*   [`rust_api_from_cpp`](../cpp/index.md) generates a Rust library from a
    `cc_library`. Load it from `@rules_crubit//bazel:rust_api_from_cpp.bzl`.

The
[`examples/build_systems/bazel`](https://github.com/google/crubit/tree/main/examples/build_systems/bazel)
project is the reference setup. CI builds it on every change to Crubit. The
fastest way to see Crubit working is to run it:

```sh
git clone https://github.com/google/crubit.git
cd crubit/examples/build_systems/bazel
bazelisk run //:main
```

The first build compiles Crubit from source, so expect it to take a while.

## Set up your module

Add `rules_crubit` to your `MODULE.bazel`, together with the Rust and LLVM
toolchains. The example project's `MODULE.bazel` shows the exact versions that
Crubit is tested with:

```
{{ #include ../../examples/build_systems/bazel/MODULE.bazel }}
```
<!--  -->


The example uses `local_path_override` because it lives inside the Crubit
repository. In your own project, use `git_override` instead, and pin a commit
(for example, the one the
[`latest`](https://github.com/google/crubit/tree/latest) tag points to):

```python
git_override(
    module_name = "rules_crubit",
    remote = "https://github.com/google/crubit.git",
    commit = "<commit SHA>",
)
```

`crubit_toolchains.configure` picks a Rust toolchain that ships the `rustc-dev`
component and a matching LLVM release. See
[Toolchain requirements](build_systems.md#toolchain-requirements) for the
versions that work.

Also copy the example's
[`.bazelrc`](https://github.com/google/crubit/blob/main/examples/build_systems/bazel/.bazelrc).
It sets the Rust channel and the C++20 and Clang flags that Crubit expects.

## C++ bindings for a Rust library {#cpp_api_from_rust}

Wrap the `rust_library` in a `cpp_api_from_rust` target, and depend on that
target from C++:

```
{{ #include ../../examples/build_systems/bazel/BUILD.bazel }}
```
<!--  -->


The header name is the Rust target's name with `.h` appended, and the
namespace is the target name:

```
{{ #include ../../examples/build_systems/bazel/main.cpp }}
```
<!--  -->


See [C++ bindings for Rust libraries](../rust/index.md) for what the generated
API looks like, and how to customize it.

## Rust bindings for a C++ library {#rust_api_from_cpp}

Define a `rust_api_from_cpp` target next to the `cc_library`, and add its
automatically generated `<name>.hint` to the `cc_library`'s `aspect_hints`:

```python
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_crubit//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("@rules_rust//rust:rust_binary.bzl", "rust_binary")

cc_library(
    name = "foo",
    hdrs = ["foo.h"],
    srcs = ["foo.cc"],
    aspect_hints = [":foo_rust.hint"],
)

rust_api_from_cpp(
    name = "foo_rust",
    cpp_target = ":foo",
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],  # use foo_rust::...;
    deps = [":foo_rust"],
)
```

See [Rust bindings for C++ libraries](../cpp/index.md) for what the generated
API looks like, and how to customize it.

<!-- TODO(b/574505910): Add rust_api_from_cpp to examples/build_systems/bazel
and cover it in CI, then switch the snippet above to a live-snippet. -->
