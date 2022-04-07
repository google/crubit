# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""
The tool creates Rust source code with the C++ API projection as well as implementation of the API
projection. See <internal link> and <internal link> for
more context.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "DepsForBindingsInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils.bzl",
    "RustBindingsFromCcInfo",
    "RustToolchainHeadersInfo",
    "bindings_attrs",
    "generate_and_compile_bindings",
)

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:providers.bzl", "DepVariantInfo")

# <internal link>/127#naming-header-files-h-and-inc recommends declaring textual headers either in the
# `textual_hdrs` attribute of the Bazel C++ rules, or using the `.inc` file extension. Therefore
# we are omitting ["inc"] from the list below.
_hdr_extensions = ["h", "hh", "hpp", "ipp", "hxx", "h++", "inl", "tlh", "tli", "H", "tcc"]

def _is_hdr(input):
    return input.path.split(".")[-1] in _hdr_extensions

def _filter_hdrs(input_list):
    return [hdr for hdr in input_list if _is_hdr(hdr)]

public_headers_to_remove = {
    "//base:base": [
        "base/callback.h",  # //base:callback
        "base/callback-specializations.h",  # //base:callback
        "base/callback-types.h",  # //base:callback
        "base/googleinit.h",  # //base:googleinit
        "base/logging.h",  # //base:logging
    ],
}

def _collect_hdrs(ctx):
    public_hdrs = _filter_hdrs(ctx.rule.files.hdrs)
    private_hdrs = _filter_hdrs(ctx.rule.files.srcs) if hasattr(ctx.rule.attr, "srcs") else []
    label = str(ctx.label)
    public_hdrs = [
        h
        for h in public_hdrs
        if h.short_path not in public_headers_to_remove.get(label, [])
    ]

    all_standalone_hdrs = public_hdrs + private_hdrs
    return public_hdrs, all_standalone_hdrs

def _rust_bindings_from_cc_aspect_impl(target, ctx):
    # We use a fake generator only when we are building the real one, in order to avoid
    # dependency cycles.
    if ctx.executable._generator.basename == "fake_rust_bindings_from_cc":
        return []

    # If this target already provides bindings, we don't need to run the bindings generator.
    if RustBindingsFromCcInfo in target:
        return []

    # We generate bindings for these headers via the
    # rs_bindings_from_cc:cc_std target.
    if target.label == Label("//third_party/stl:stl"):
        return [ctx.attr._std[RustBindingsFromCcInfo]]

    # This is not a C++ rule
    if CcInfo not in target:
        return []

    if not hasattr(ctx.rule.attr, "hdrs"):
        return []

    public_hdrs, all_standalone_hdrs = _collect_hdrs(ctx)

    # At execution time we convert this depset to a json array that gets passed to our tool through
    # the --targets_and_headers flag.
    # We can improve upon this solution if:
    # 1. we use a library for parsing command line flags that allows repeated flags.
    # 2. instead of json string, we use a struct that will be expanded to flags at execution time.
    #    This requires changes to Bazel.
    targets_and_headers = depset(
        direct = [
            json.encode({
                "t": str(ctx.label),
                "h": [h.short_path for h in all_standalone_hdrs],
            }),
        ] if all_standalone_hdrs else [],
        transitive = [
            t[RustBindingsFromCcInfo].targets_and_headers
            for t in ctx.rule.attr.deps
            if RustBindingsFromCcInfo in t
        ] + [
            # TODO(b/217667751): This is a huge list of headers; pass it as a file instead;
            ctx.attr._std[RustBindingsFromCcInfo].targets_and_headers,
        ],
    )

    if not public_hdrs:
        empty_cc_info = CcInfo()
        return RustBindingsFromCcInfo(
            cc_info = empty_cc_info,
            dep_variant_info = DepVariantInfo(cc_info = empty_cc_info),
            targets_and_headers = targets_and_headers,
        )

    header_includes = []
    for hdr in public_hdrs:
        header_includes.append("-include")
        header_includes.append(hdr.short_path)

    return generate_and_compile_bindings(
        ctx,
        ctx.rule.attr,
        compilation_context = target[CcInfo].compilation_context,
        public_hdrs = public_hdrs,
        header_includes = header_includes,
        action_inputs = depset(
            direct = public_hdrs + ctx.files._builtin_hdrs,
            transitive = [
                ctx.attr._std[RustToolchainHeadersInfo].headers,
            ],
        ),
        targets_and_headers = targets_and_headers,
        deps_for_cc_file = [target[CcInfo]] + [
            dep[RustBindingsFromCcInfo].cc_info
            for dep in ctx.rule.attr.deps
            if RustBindingsFromCcInfo in dep
        ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_cc_file + [
            ctx.attr._std[RustBindingsFromCcInfo].cc_info,
        ],
        deps_for_rs_file = [
            dep[RustBindingsFromCcInfo].dep_variant_info
            for dep in ctx.rule.attr.deps
            if RustBindingsFromCcInfo in dep
        ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_rs_file + [
            ctx.attr._std[RustBindingsFromCcInfo].dep_variant_info,
        ],
    )

rust_bindings_from_cc_aspect = aspect(
    implementation = _rust_bindings_from_cc_aspect_impl,
    attr_aspects = ["deps"],
    attrs = dict(bindings_attrs.items() + {
        "_std": attr.label(
            default = "//rs_bindings_from_cc:cc_std",
        ),
    }.items()),
    toolchains = [
        "@rules_rust//rust:toolchain",
        "@bazel_tools//tools/cpp:toolchain_type",
    ],
    host_fragments = ["cpp"],
    fragments = ["cpp", "google_cpp"],
)
