# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Generates Rust bindings for a C++ target such as a `cc_library`.

This acts like a `rust_library`, and may be listed in `deps` of a rust rule.

The C++ target must enable bindings generation by adding the automatically
generated `<name>.hint` target to its `aspect_hints` attribute.

Example:
```python
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

cc_library(
    name = "foo",
    aspect_hints = [":foo_rust.hint"],
)

rust_api_from_cpp(
    name = "foo_rust",
    cpp_target = ":foo",
)
```

By default, `bazel build` will compile the bindings.
You can request bindings generation only via `--output_groups=sources`.
"""

load("@rules_rust//rust:rust_common.bzl", "CrateInfo", "DepInfo")
load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl", "additional_rust_srcs_for_crubit_bindings")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:providers.bzl", "GeneratedBindingsInfo", "RustBindingsFromCcInfo")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl", "rust_bindings_from_cc_aspect")

def _rust_api_from_cpp_generate_bindings_impl(ctx):
    """Implementation of the `rust_bindings_from_cc` rule."""

    variant = ctx.attr.target[RustBindingsFromCcInfo].dep_variant_info
    if not variant or not OutputGroupInfo in ctx.attr.target:
        fail("No bindings generated for target: %s" % ctx.attr.target.label)
    providers = [
        variant.crate_info,
        variant.dep_info,
        variant.cc_info,
        DefaultInfo(files = ctx.attr.target[OutputGroupInfo]["out_compiled"]),
        OutputGroupInfo(sources = ctx.attr.target[OutputGroupInfo]["out"]),
    ]
    if GeneratedBindingsInfo in ctx.attr.target:
        providers.append(ctx.attr.target[GeneratedBindingsInfo])
    if CcBindingsFromRustInfo in ctx.attr.target:
        providers.append(ctx.attr.target[CcBindingsFromRustInfo])
    return providers

_rust_api_from_cpp_generate_bindings = rule(
    implementation = _rust_api_from_cpp_generate_bindings_impl,
    doc = "Generates Rust bindings for a C++ target.",
    provides = [CrateInfo, DepInfo],
    attrs = {
        "target": attr.label(
            doc = "The C++ target to generate bindings for.",
            allow_files = False,
            mandatory = True,
            providers = [RustBindingsFromCcInfo],
            aspects = [rust_bindings_from_cc_aspect],
        ),
    },
)

def rust_api_from_cpp(
        name,
        cpp_target,
        srcs = [],
        compile_data = [],
        cpp_srcs = [],
        deps = [],
        link_deps = [],
        srcs_mod = "",
        root_namespaces = [],
        generated_cpp_support_deps = [],
        unstable_rust_features = [],
        aliases = {},
        warn_on_raw_cc_deps = True,
        **kwargs):
    """Generates Rust bindings for a C++ target.

    This macro automatically generates an aspect hint target named `<name>.hint`
    (e.g., `foo_rust.hint` if `name` is `foo_rust`). This hint target MUST be
    manually added to the `aspect_hints` attribute of the C++ target specified
    in `cpp_target` to enable bindings generation and pass configuration to the
    bindings generator.

    Args:
      name: The name of the rule.
      cpp_target: The C++ target to generate bindings for.
      srcs: Handwritten Rust code to merge into the bindings. Note: to allow
        Crubit bindings to be `#![no_std]` by default, these files do not include
        the regular `std` Rust prelude, meaning that types like `Box` have to be
        imported manually: `extern crate std; use std::boxed::Box;`.
      compile_data: List of files used by `srcs` at compile time (e.g. via
        `include_str!` or `include_bytes!`). These files can be referenced from
        `srcs` using paths relative to the source file, just like the
        `compile_data` attribute of `rust_library`.
      cpp_srcs: Additional C++ source files (e.g. from inline_cpp!).
      deps: Dependencies for the Rust API.
      link_deps: Native C++ dependencies to link into the Rust library.
      srcs_mod: Controls which module inside the Rust crate the additional rust sources are added to.
      root_namespaces: List of C++ namespaces to re-export at the root of the generated Rust bindings.
      generated_cpp_support_deps: C++ dependencies of the generated C++ target (e.g. ABI support libraries required by the generated bindings).
      unstable_rust_features: List of unstable rustc features to enable via `#![feature(...)]`.
      aliases: A dictionary mapping targets (C++ or Rust) to their crate names for renaming.
            The keys can be Rust libraries, or either C++ libraries (e.g. `cc_library`) or their corresponding
            generated Rust bindings (e.g. `rust_api_from_cpp`).
      warn_on_raw_cc_deps: Whether to emit a warning when a direct dependency in `deps` is a raw C++ library without Rust bindings.
      **kwargs: Additional arguments passed to the underlying rule.
    """

    hint_kwargs = {}
    if "compatible_with" in kwargs:
        hint_kwargs["compatible_with"] = kwargs["compatible_with"]
    if "restricted_to" in kwargs:
        hint_kwargs["restricted_to"] = kwargs["restricted_to"]
    if "testonly" in kwargs:
        hint_kwargs["testonly"] = kwargs["testonly"]

    hint_name = name + ".hint"
    if not srcs and not compile_data and not cpp_srcs and not deps and not link_deps and not root_namespaces:
        native.alias(
            name = hint_name,
            actual = "//features:supported",
            **hint_kwargs
        )
    else:
        additional_rust_srcs_for_crubit_bindings(
            name = hint_name,
            srcs = srcs,
            compile_data = compile_data,
            cpp_srcs = cpp_srcs,
            deps = [d for d in deps if d != cpp_target],
            link_deps = [d for d in link_deps if d != cpp_target],
            namespace_path = srcs_mod,
            root_namespaces = root_namespaces,
            generated_cpp_support_deps = generated_cpp_support_deps,
            unstable_rust_features = unstable_rust_features,
            aliases = aliases,
            warn_on_raw_cc_deps = warn_on_raw_cc_deps,
            **hint_kwargs
        )

    # Note: This macro relies on the user attaching `hint_name` to their `cpp_target`'s `aspect_hints` manually.

    _rust_api_from_cpp_generate_bindings(
        name = name,
        target = cpp_target,
        **kwargs
    )
