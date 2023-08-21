# Crubit and Crosstool release

<!--*
# Document freshness: For more information, see <internal link>.
freshness: { owner: 'lujin' reviewed: '2023-08-18' }
*-->

[TOC]

TODO(b/274102056): Write summary on this docs when close to completion.

The bindings generators components of Crubit are being released, and it's
released by [the Crosstool release](<internal link>) process.

Specifically, bindings generators components that are in Crosstool release are:

*   The bindings generator tools:

    *   //rs_bindings_from_cc:rs_bindings_from_cc
    *   //cc_bindings_from_rs:cc_bindings_from_rs

    They are built from the targets above and released at
    google3/nowhere/llvm/rust/crubit/bin/.

*   Crubit support libraries: support/.

    They are copied as source files and transformed and are released at
    google3/nowhere/llvm/rust/crubit/support/.

This means that when you `bazel build`, say a `rust_binary` with `cc_deps`:

*   The "released" version of the `cc_bindings_from_rs` bindings generator tool
    in `third_party/crosstool` is invoked to generate the Rust binding, instead
    of the one freshly-built from  with the latest
    and your workspace changes.

*   The "released" version of the Crubit support libraries are used in your
    bindings.

## Documentation for Crubit developers

The following sections are meant for Crubit developers.

### Different Crubit flavors

Concretely, a "Crubit flavor" is a combination of build flag values that impacts
which Crubit is used and how Crubit is built.

#### Flags that control "Crubit flavor"

2 boolean flags determine flavor of Crubit:

*   `//google_internal/build_flavors:on_demand`: if true, use
    Crubit built from your workspace which has your local changes, otherwise,
    use the prebuilt binding generators in one of the Crosstool directories.
    `--config=crubit-on-demand` for short.

*   `//third_party/crosstool:unstable_flag`: if true, use Crosstool unstable
    toolchains in all build actions - notably, the actions that build Crubit
    binding generators (if on-demand), the actions that build Rust crates (e.g.,
    for a `rust_binary` with `cc_deps`) or C++ libraries, otherwise use the
    Crosstool stable toolchains. `--config=llvm-unstable` for short.

#### 4 flavors and where they are useful

The combination of the 2 boolean flags results in the 4 following Crubit
flavors.
(google_internal/build_flavors/crubit_build_flavor_defs.bzl
has the full specification and exports `crubit_build_flavors` for easier
reference.)

`stable`
:   Uses the prebuilt Crubit in Crosstool stable directory, which gets released
    by rapid/llvm_crosstool ~weekly.

    This is also the `default` flavor of Crubit.

`llvm_unstable`
:   Uses the prebuilt Crubit in Crosstool unstable directory.
    rapid/rust_crosstool_unstable builds Crosstool unstable Crubit bindings
    generators using the freshly bootstrapped "unstable" rust toolchain ~daily.

    This flavor is useful for exercising the process of rebuilding Crubit
    binding generators and branching the support library source file, before the
    Crosstool stable release. For example, it can catch the case that the
    dependency of Crubit support libraries isn't visible from Crosstool
    directories.

`on_demand_built_with_llvm_unstable`
:   Uses the Crubit built from your workspace that has your local changes using
    the Crosstool unstable toolchains (rustc, rustc-as-a-library, clang, etc.)
    throughout.

    This will soon be the default flavor for Crubit *development*.

    This flavor is useful for detecting upstream rustc changes that are
    incompatible with Crubit, before Crosstool release: Crubit depends closely
    on rustc-as-a-library API, which isn't stable; upstream rustc library
    changes are first surfaced through Crosstool unstable rust.

`on_demand_built_with_llvm_stable`
:   Similar to `on_demand_built_with_llvm_unstable`, except that Crosstool
    stable toolchains (rustc, rustc-as-a-library, clang, etc.) is used
    throughout the build.

    This flavor combination exists, but it won't be exercised by TAP/Guitar. It
    may become helpful as a backup for local development in case Crosstool
    unstable toolchains are broken.
