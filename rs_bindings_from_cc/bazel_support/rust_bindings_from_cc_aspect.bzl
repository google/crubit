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
    "@@//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "get_additional_rust_srcs",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "find_crubit_features",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "DepsForBindingsInfo",
    "RustBindingsFromCcInfo",
    "RustToolchainHeadersInfo",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_cli_flag_aspect_hint.bzl",
    "collect_rust_bindings_from_cc_cli_flags",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils.bzl",
    "bindings_attrs",
    "generate_and_compile_bindings",
)

# <internal link>/127#naming-header-files-h-and-inc recommends declaring textual headers either in the
# `textual_hdrs` attribute of the Bazel C++ rules, or using the `.inc` file extension. Therefore
# we are omitting ["inc"] from the list below.
_hdr_extensions = ["h", "hh", "hpp", "ipp", "hxx", "h++", "inl", "tlh", "tli", "H", "tcc"]

def _is_hdr(input):
    return input.path.split(".")[-1] in _hdr_extensions

def _filter_hdrs(input_list):
    return [hdr for hdr in input_list if _is_hdr(hdr)]

# Targets which do not receive rust bindings at all.
targets_to_remove = [
]

# Specific headers, in specific targets, which do not receive Rust bindings.
#
# This is mainly for if the same header is in two different targets, only one of which is canonical.
public_headers_to_remove = {
}
private_headers_to_remove = {
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
    private_hdrs = [
        h
        for h in private_hdrs
        if h.short_path not in private_headers_to_remove.get(label, [])
    ]

    all_standalone_hdrs = depset(public_hdrs + private_hdrs).to_list()
    return public_hdrs, all_standalone_hdrs

def _is_proto_library(target):
    return ProtoInfo in target

def _is_cc_proto_library(rule):
    return rule.kind == "cc_proto_library"

def retain_proto_dot_h_headers(headers):
    return [h for h in headers if h.path.endswith("proto.h")]

def _rust_bindings_from_cc_aspect_impl(target, ctx):
    # We use a fake generator only when we are building the real one, in order to avoid
    # dependency cycles.
    if ctx.executable._generator.basename == "fake_rust_bindings_from_cc":
        return []

    # If this target already provides bindings, we don't need to run the bindings generator.
    if RustBindingsFromCcInfo in target:
        return []

    # We generate bindings for these headers via the
    # support/cc_std:cc_std target.
    if target.label == Label("//third_party/stl:stl"):
        return [ctx.attr._std[RustBindingsFromCcInfo]]

    # This is not a C++ rule
    if CcInfo not in target:
        return []

    if _is_cc_proto_library(ctx.rule):
        # This is cc_proto_library, we are interested in RustBindingsFromCcInfo provider of the
        # proto_library.
        return [ctx.rule.attr.deps[0][RustBindingsFromCcInfo]]

    if str(ctx.label) in targets_to_remove:
        return []

    extra_cc_compilation_action_inputs = []
    extra_rule_specific_deps = []
    public_hdrs = []
    all_standalone_hdrs = []
    if hasattr(ctx.rule.attr, "hdrs"):
        public_hdrs, all_standalone_hdrs = _collect_hdrs(ctx)
    elif _is_proto_library(target):
        #TODO(b/232199093): Ideally we would get this information from a proto-specific provider,
        # but ProtoCcFilesProvider is private currently. Use it once public.
        public_hdrs = retain_proto_dot_h_headers(target[CcInfo].compilation_context.direct_headers)
        all_standalone_hdrs = public_hdrs
        extra_rule_specific_deps = [ctx.rule.attr._cc_lib]
    elif ctx.rule.kind == "cc_embed_data" or ctx.rule.kind == "upb_proto_library":
        public_hdrs = target[CcInfo].compilation_context.direct_public_headers
        all_standalone_hdrs = target[CcInfo].compilation_context.direct_headers

    if not public_hdrs:
        # This target doesn't have public headers, so there are no bindings to generate. However we
        # still need to propagate dependencies since not every C++ target is layering check clean.
        # Since there is no existing API to merge Rust providers besides calling
        # `rustc_compile_action`, we decided to create an empty file and compile it.
        empty_header_file = ctx.actions.declare_file(ctx.label.name + ".empty_source_no_public_headers.h")
        ctx.actions.write(
            empty_header_file,
            "// File intentionally left empty, its purpose is to satisfy rules_rust APIs.",
        )
        public_hdrs = [empty_header_file]
        all_standalone_hdrs = public_hdrs
        extra_cc_compilation_action_inputs = public_hdrs

    all_deps = getattr(ctx.rule.attr, "deps", []) + extra_rule_specific_deps + [
        # TODO(b/217667751): This contains a huge list of headers_and_targets; pass them as a file
        # instead.
        ctx.attr._std,
    ]

    # At execution time we convert this depset to a json array that gets passed to our tool through
    # the --target_args flag.
    # We can improve upon this solution if:
    # 1. we use a library for parsing command line flags that allows repeated flags.
    # 2. instead of json string, we use a struct that will be expanded to flags at execution time.
    #    This requires changes to Bazel.
    direct_target_args = {}
    features = find_crubit_features(target, ctx)
    if all_standalone_hdrs:
        direct_target_args["h"] = [h.path for h in all_standalone_hdrs]
    if features:
        direct_target_args["f"] = features

    if direct_target_args:
        direct_target_args["t"] = str(ctx.label)
        direct = [json.encode(direct_target_args)]
    else:
        direct = []

    target_args = depset(
        direct = direct,
        transitive = [
            t[RustBindingsFromCcInfo].target_args
            for t in all_deps
            if RustBindingsFromCcInfo in t
        ],
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
        target_args = target_args,
        extra_rs_srcs = get_additional_rust_srcs(target, ctx),
        deps_for_cc_file = [target[CcInfo]] + [
            dep[RustBindingsFromCcInfo].cc_info
            for dep in all_deps
            if RustBindingsFromCcInfo in dep and
               dep[RustBindingsFromCcInfo].cc_info
        ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_cc_file,
        deps_for_rs_file = [
            dep[RustBindingsFromCcInfo].dep_variant_info
            for dep in all_deps
            if RustBindingsFromCcInfo in dep
        ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_rs_file,
        extra_cc_compilation_action_inputs = extra_cc_compilation_action_inputs,
        extra_rs_bindings_from_cc_cli_flags = collect_rust_bindings_from_cc_cli_flags(target, ctx),
    )

rust_bindings_from_cc_aspect = aspect(
    implementation = _rust_bindings_from_cc_aspect_impl,
    attr_aspects = [
        # for cc_library and similar rules
        "deps",
        # for cc_proto_aspect implicit deps
        "_cc_lib",
    ],
    required_aspect_providers = [CcInfo],
    attrs = dict(bindings_attrs.items() + {
        "_std": attr.label(
            default = "//support/cc_std",
        ),
    }.items()),
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        "@bazel_tools//tools/cpp:toolchain_type",
    ],
    host_fragments = ["cpp"],
    fragments = ["cpp", "google_cpp"],
)
