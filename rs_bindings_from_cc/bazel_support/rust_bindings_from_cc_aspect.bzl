# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
The tool creates Rust source code with the C++ API projection as well as implementation of the API
projection. See <internal link> and <internal link> for
more context.
"""

load(
    "//features:crubit_feature_hint.bzl",
    "find_crubit_features",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "get_additional_rust_srcs",
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
load("@protobuf//rust:aspects.bzl", "rust_cc_proto_library_aspect")

# <internal link>/127#naming-header-files-h-and-inc recommends declaring textual headers either in the
# `textual_hdrs` attribute of the Bazel C++ rules, or using the `.inc` file extension. Therefore
# we are omitting ["inc"] from the list below.
_hdr_extensions = ["h", "hh", "hpp", "ipp", "hxx", "h++", "inl", "tlh", "tli", "H", "tcc"]

def _is_hdr(input):
    return input.path.split(".")[-1] in _hdr_extensions

def _filter_hdrs(input_list):
    return [hdr for hdr in input_list if _is_hdr(hdr)]

# Targets which do not receive rust bindings at all. Most significantly, the header is not
# attributed to belonging to this target. So, the main use for this list is to resolve
# ambiguously-owned headers by disabling one of the targets.
targets_to_remove = [
]

# Specific headers, in specific targets, which do not receive Rust bindings.
#
# This is mainly for if the same header is in two different targets, only one of which is canonical.
public_headers_to_remove = {
}
private_headers_to_remove = {
}

def _collect_hdrs(ctx, crubit_features):
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

    # If Crubit is not enabled for this target, then disable header parsing by removing all headers
    # from the list of public headers. This allows Crubit to work on target A, even if it
    # transitively depends on a target B which would cause Crubit to crash (e.g. because the headers
    # are unparseable).
    #
    # Note: We cannot e.g. check `if "parse_headers" in ctx.disabled_features:`, because some build
    # configurations (like AddressSanitizer) set `-parse_headers`, even if the headers are
    # parseable, just to save work. So if we want to avoid attempting to parse unparseable headers,
    # we must for now make the worst-case assumption that anything which does not explicitly support
    # Crubit cannot work with Crubit.
    #
    # In principle, we can modify bazel someday to allow us to detect when parse_headers is disabled
    # for a target in all build configurations, instead of as part of a build configuration like
    # AddressSanitizer.
    if not crubit_features:
        public_hdrs = []
    return public_hdrs, all_standalone_hdrs

def _is_cc_proto_library(rule):
    return rule.kind == "cc_proto_library"

def retain_proto_dot_h_headers(headers):
    return [h for h in headers if h.path.endswith("proto.h")]

def _rust_bindings_from_cc_aspect_impl(target, ctx):
    # Faithless is he that says farewell when the road darkens (=Fasten the seatbelt).
    #
    # rust_bindings_from_cc_aspect requires cc_proto_aspect (because it visits through CcInfo), and
    # it requires rust_cc_proto_library_aspect (because we need to get a hand at the Protobuf Rust
    # generated crates). Also, rust_cc_proto_library_aspect requires cc_proto_aspect (because
    # Protobuf Rust gencode builds on top of C++ protobufs). Let's sketch a hypothethical example:
    # +----------------+
    # |rust_library    |
    # | :server_handler|
    # |  (0 aspects)   |
    # +---+------------+-------------+
    #     |deps                      | cc_deps
    # +---v---------+          +-----v-------+
    # |rust_library |          |cc_library   |
    # | :utils      |          | :cc_utils   |
    # |  (0 aspects)|          |  (3 aspects)|
    # +----+--------+          +-------+-----+
    #      |cc_deps                    | deps
    #  +---v----------+                |
    #  |cc_library    |         +------v---------+
    #  | absl/time    |         |cc_proto_library|
    #  |   (3 aspects)|         | :my_cc_proto   |
    #  +--------------+         |   (3 aspects)  |
    #                           +--+-------------+----------+
    #                              |deps                    |_cc_lib
    #                         +----v---------+       +------v------+
    #                         |proto_library |       |cc_library   |
    #                         | :my_proto    |       | :pb_runtime |
    #                         |  (3 aspects) |       |  (2 aspects)|
    #                         +--------------+       +-------+-----+
    #                                                        |deps
    #                                                 +------v------+
    #                                                 |cc_library   |
    #                                                 | absl/time   |
    #                                                 |  (2 aspects)|
    #                                                 +-------------+
    #
    # So, rust_cc_proto_library_aspect + rust_cc_proto_library_aspect + cc_proto_aspect are all
    # attached to dependencies through `rust_library.cc_deps` attribute.

    # cc_proto_aspect implicitly depends on C++ Protobuf runtime library through its `_cc_lib`
    # attribute. Transitively, the runtime depends on //third_party/absl. //third_party/absl is also
    # depended on by other paths in the build graph.
    #
    # `_cc_lib` is the root of the problem. We're asking Bazel to attach cc_proto_aspect onto an
    # implicit dependency of itself. That's a dependency cycle. Bazel
    # solves it by silently removing cc_proto_aspect from the set of aspects, and attaching this
    # smaller set onto `_cc_lib`` transitively. In our example (and almost always in large builds),
    # there are multiple paths through the dependency graph of the target through some
    # cc_proto_library, and without a cc_proto_library, that land at a common foundational libraries
    # such as absl. So, Bazel will generate 2 shadow targets for absl. One with 3 aspects, one with
    # 2 aspects.
    #
    # This is quite a pickle on so many levels, but most immediately this results in action
    # conflicts as rust_bindings_from_cc_aspect - it registers exactly the same binding generation
    # actions for the shadow target for 2 aspects and for the shadow target for 3 aspects.
    #
    # The fix, ugly as it is, is to check if cc_proto_aspect is present in the aspect ids. If not,
    # we are in the shadow target for 2 aspects, and we can return early. This is only possible
    # because:
    # 1. We don't need Crubit bindings for `_cc_lib` for protobuf interop, we use protoc for that.
    # 2. We know that transitive deps of `_cc_lib` will get Crubit bindings through the "3 aspects"
    #    path if they are needed.
    if "@protobuf//bazel/private:google_cc_proto_library.bzl%cc_proto_aspect" not in ctx.aspect_ids:
        return []

    # We use a fake generator only when we are building the real one, in order to avoid
    # dependency cycles.
    toolchain = ctx.toolchains["@@//rs_bindings_from_cc/bazel_support:toolchain_type"].rs_bindings_from_cc_toolchain_info
    generator = toolchain.binary
    if generator.basename == "fake_rust_bindings_from_cc":
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
    features = find_crubit_features(target, ctx)
    if hasattr(ctx.rule.attr, "hdrs"):
        public_hdrs, all_standalone_hdrs = _collect_hdrs(ctx, features)

    elif ctx.rule.kind == "cc_embed_data" or ctx.rule.kind == "upb_proto_library":
        public_hdrs = target[CcInfo].compilation_context.direct_public_headers
        all_standalone_hdrs = target[CcInfo].compilation_context.direct_headers

    elif ctx.rule.kind == "cc_stubby_library":
        public_hdrs = target[CcInfo].compilation_context.direct_public_headers
        all_standalone_hdrs = target[CcInfo].compilation_context.direct_headers
        extra_rule_specific_deps = ctx.rule.attr.implicit_cc_deps

    has_public_headers = len(public_hdrs) > 0
    if not has_public_headers:
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
        all_standalone_hdrs = all_standalone_hdrs + public_hdrs
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
        # Use full `path`, instead of `short_path`, so that generated headers (e.g.,
        # `empty_source_no_public_headers.h`) can be found.
        header_includes.append("-include")
        header_includes.append(hdr.path)

    extra_rs_srcs = get_additional_rust_srcs(ctx)
    binding_infos = [
        dep[RustBindingsFromCcInfo]
        for dep in all_deps
        if RustBindingsFromCcInfo in dep
    ]
    return generate_and_compile_bindings(
        ctx,
        ctx.rule.attr,
        compilation_context = target[CcInfo].compilation_context,
        public_hdrs = public_hdrs,
        header_includes = header_includes,
        action_inputs = depset(
            direct = public_hdrs + toolchain.builtin_headers,
            transitive = [
                ctx.attr._std[RustToolchainHeadersInfo].headers,
            ],
        ),
        target_args = target_args,
        extra_rs_srcs = extra_rs_srcs,
        deps_for_cc_file = [target[CcInfo]] + [
            d.cc_info
            for d in binding_infos
            if d.cc_info
        ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_cc_file,
        deps_for_rs_file = depset(
            direct = [
                d.dep_variant_info
                for d in binding_infos
                if d.dep_variant_info
            ] + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_rs_file,
            transitive = [
                d.pass_through_dep_variant_infos
                for d in binding_infos
            ],
        ),
        extra_cc_compilation_action_inputs = extra_cc_compilation_action_inputs,
        extra_rs_bindings_from_cc_cli_flags = collect_rust_bindings_from_cc_cli_flags(target, ctx),
        should_generate_bindings = (
            has_public_headers or extra_rs_srcs
        ) and not _is_cc_proto_library(target),
    )

rust_bindings_from_cc_aspect = aspect(
    implementation = _rust_bindings_from_cc_aspect_impl,
    attr_aspects = [
        # for cc_library and similar rules
        "deps",
        # for cc_proto_aspect implicit deps
        "_cc_lib",
        # for cc_stubby_library implicit deps
        "implicit_cc_deps",
    ],
    requires = [rust_cc_proto_library_aspect],
    required_aspect_providers = [CcInfo],
    attrs = bindings_attrs | {
        "_std": attr.label(
            default = "//support/cc_std",
        ),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        "@bazel_tools//tools/cpp:toolchain_type",
        "@@//rs_bindings_from_cc/bazel_support:toolchain_type",
    ],
    fragments = ["cpp", "google_cpp"],
)
