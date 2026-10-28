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

load("@rules_cc//cc/common:cc_common.bzl", "cc_common")
load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load("@rules_rust//rust:rust_common.bzl", "CrateInfo", "DepInfo")
load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl", "additional_rust_srcs_for_crubit_bindings")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:providers.bzl", "GeneratedBindingsInfo", "RustBindingsFromCcInfo")
load("@rules_crubit//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl", "rust_bindings_from_cc_aspect")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

def _rust_api_from_cpp_generate_bindings_impl(ctx):
    """Implementation of the `rust_bindings_from_cc` rule."""

    variant = ctx.attr.target[RustBindingsFromCcInfo].dep_variant_info
    if not variant or not OutputGroupInfo in ctx.attr.target:
        fail("No bindings generated for target: %s" % ctx.attr.target.label)
    extra_cc_infos = [dep[CcInfo] for dep in ctx.attr.cpp_api_deps]
    cc_info = variant.cc_info
    if extra_cc_infos:
        cc_info = cc_common.merge_cc_infos(
            direct_cc_infos = extra_cc_infos,
            cc_infos = [cc_info],
        )
    providers = [
        variant.crate_info,
        variant.dep_info,
        cc_info,
        DefaultInfo(files = ctx.attr.target[OutputGroupInfo]["out_compiled"]),
        OutputGroupInfo(sources = ctx.attr.target[OutputGroupInfo]["out"]),
    ]
    if GeneratedBindingsInfo in ctx.attr.target:
        providers.append(ctx.attr.target[GeneratedBindingsInfo])
    if CcBindingsFromRustInfo in ctx.attr.target:
        cc_bindings_info = ctx.attr.target[CcBindingsFromRustInfo]
        if extra_cc_infos:
            cc_bindings_info = CcBindingsFromRustInfo(
                cc_info = cc_common.merge_cc_infos(
                    direct_cc_infos = extra_cc_infos,
                    cc_infos = [cc_bindings_info.cc_info],
                ),
                crate_key = cc_bindings_info.crate_key,
                headers = cc_bindings_info.headers,
                features = cc_bindings_info.features,
                target_denied_features = getattr(cc_bindings_info, "target_denied_features", []),
                configuration = cc_bindings_info.configuration,
                transitive_infos = cc_bindings_info.transitive_infos,
            )
        providers.append(cc_bindings_info)
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
        "cpp_api_deps": attr.label_list(
            doc = "Additional C++ dependencies propagated to C++ bindings generated from Rust crates that depend on this target.",
            providers = [CcInfo],
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
        cpp_api_deps = [],
        srcs_mod = "",
        root_namespaces = [],
        generated_cpp_support_deps = [],
        unstable_rust_features = [],
        crubit_features = [],
        types_only = False,
        aliases = {},
        warn_on_raw_cc_deps = True,
        embedded_cpp = False,
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
      cpp_srcs: Additional C++ source files (e.g. from inline_cpp!). Note: if
        `cpp_srcs` is non-empty, the generated `_rust_api_impl.cc` is compiled
        textually, without precompiled header modules (slower) and without
        `layering_check` (b/573160050).
        Other than the wrapped library's own headers, headers included by
        `cpp_srcs` should come from `generated_cpp_support_deps`.
      deps: Dependencies for the Rust API.
      link_deps: Native C++ dependencies to link into the Rust library.
      cpp_api_deps: Additional C++ dependencies propagated to `cpp_api_from_rust`
        bindings for Rust crates that depend on this target (for example, C++
        libraries whose headers are referenced via `CRUBIT_ANNOTATE: include_path=...`
        in `srcs` and cannot be added to `cpp_target` or `<name>.hint` without
        creating a dependency cycle).
      srcs_mod: Controls which module inside the Rust crate the additional rust sources are added to.
      root_namespaces: List of C++ namespaces to re-export at the root of the generated Rust bindings.
      generated_cpp_support_deps: C++ dependencies of the generated C++ target (e.g. ABI support libraries required by the generated bindings).
      unstable_rust_features: List of unstable rustc features to enable via `#![feature(...)]`.
      crubit_features: List of additional Crubit features to enable on top of
        `supported` for bindings generation (e.g. `["infer_send_sync"]` or
        `["experimental"]`). Valid features are defined in
        `//features:BUILD`.
      types_only: If True, only generates bindings for types
        (`//features:types`), disabling `supported` features
        such as named functions and methods. Cannot be combined with
        `crubit_features`.
      aliases: A dictionary mapping targets (C++ or Rust) to their crate names for renaming.
            The keys can be Rust libraries, or either C++ libraries (e.g. `cc_library`) or their corresponding
            generated Rust bindings (e.g. `rust_api_from_cpp`).
      warn_on_raw_cc_deps: Whether to emit a warning when a direct dependency in `deps` is a raw C++ library without Rust bindings.
      embedded_cpp: If True, `srcs` may contain `global_cpp!` and `inline_cpp!` blocks. The
        embedded C++ is extracted and added to `cpp_srcs` (see there for the build cost).
        `inline_cpp!` in `srcs` fails to compile without it. `global_cpp!` blocks must
        `#include` every header the embedded C++ uses, including the headers of `cpp_target`.
        Include only headers of `cpp_target` or `generated_cpp_support_deps`: other headers
        build today, but will break once `cpp_srcs` is compiled with `layering_check`
        (b/573160050).
      **kwargs: Additional arguments passed to the underlying rule.
    """

    hint_kwargs = {}
    if "compatible_with" in kwargs:
        hint_kwargs["compatible_with"] = kwargs["compatible_with"]
    if "restricted_to" in kwargs:
        hint_kwargs["restricted_to"] = kwargs["restricted_to"]
    if "testonly" in kwargs:
        hint_kwargs["testonly"] = kwargs["testonly"]

    if "supported" in crubit_features or "//features:supported" in crubit_features:
        fail("In rust_api_from_cpp target '%s': 'supported' is enabled by default and should not be specified in `crubit_features`." % name)
    if "types" in crubit_features or "//features:types" in crubit_features:
        fail("In rust_api_from_cpp target '%s': use `types_only = True` instead of specifying 'types' in `crubit_features`." % name)
    if types_only and crubit_features:
        fail("In rust_api_from_cpp target '%s': `types_only = True` cannot be combined with `crubit_features`." % name)

    if types_only:
        crubit_features = ["//features:types"]
    elif not crubit_features:
        crubit_features = ["//features:supported"]
    else:
        crubit_features = [
            f if (f.startswith("//") or f.startswith(":")) else "//features:" + f
            for f in crubit_features
        ]

    target_label = "//{}:{}".format(native.package_name(), name)

    # Must match the `target` passed to `extract_cpp`: `inline_cpp!` hashes it into thunk names.
    crubit_target = target_label if embedded_cpp else ""
    if embedded_cpp:
        if not srcs:
            fail("In rust_api_from_cpp target '%s': `embedded_cpp = True` requires `srcs`." % name)
        extracted_cpp = name + "_embedded_cpp"
        extract_cpp(
            name = extracted_cpp,
            srcs = srcs,
            out = extracted_cpp + ".cc",
            target = target_label,
            **hint_kwargs
        )
        cpp_srcs = cpp_srcs + [":" + extracted_cpp]

    hint_name = name + ".hint"
    if not srcs and not compile_data and not cpp_srcs and not deps and not link_deps and not root_namespaces and len(crubit_features) == 1:
        native.alias(
            name = hint_name,
            actual = crubit_features[0],
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
            crubit_features = crubit_features,
            aliases = aliases,
            warn_on_raw_cc_deps = warn_on_raw_cc_deps,
            crubit_target = crubit_target,
            **hint_kwargs
        )

    # Note: This macro relies on the user attaching `hint_name` to their `cpp_target`'s `aspect_hints` manually.

    _rust_api_from_cpp_generate_bindings(
        name = name,
        target = cpp_target,
        cpp_api_deps = cpp_api_deps,
        **kwargs
    )
