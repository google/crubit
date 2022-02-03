# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils.bzl",
    "bindings_attrs",
    "generate_and_compile_bindings",
)

def _is_public_std_header(input, public_hdrs):
    return ("grte" not in input.path and
            input.basename in public_hdrs and
            "experimental" not in input.short_path)

def _collect_std_hdrs(input_list, public_hdrs):
    return [hdr for hdr in input_list if _is_public_std_header(hdr, public_hdrs)]

def _collect_nonstd_hdrs(input_list, public_hdrs):
    return [hdr for hdr in input_list if not _is_public_std_header(hdr, public_hdrs)]

def _bindings_for_toolchain_headers_impl(ctx):
    std_hdrs = ctx.files.hdrs

    # The clang builtin headers also contain some standard headers. We'll consider those part of
    # the C++ Standard library target, so we generate bindings for them.
    builtin_std_hdrs = _collect_std_hdrs(ctx.files._builtin_hdrs, ctx.attr.public_hdrs)
    builtin_nonstd_hdrs = _collect_nonstd_hdrs(
        ctx.files._builtin_hdrs,
        ctx.attr.public_hdrs,
    )

    targets_and_headers = depset(
        direct = [
            json.encode({
                "t": str(ctx.label),
                "h": [hdr.path for hdr in std_hdrs + builtin_std_hdrs],
            }),
            json.encode({
                "t": "//:_builtin_hdrs",
                "h": [h.path for h in builtin_nonstd_hdrs],
            }),
        ],
    )

    public_std_hdrs = _collect_std_hdrs(std_hdrs, ctx.attr.public_hdrs)

    header_includes = []
    for hdr in public_std_hdrs:
        header_includes.append("-include")
        header_includes.append(hdr.basename)

    compilation_context = cc_common.create_compilation_context()

    return generate_and_compile_bindings(
        ctx,
        ctx.attr,
        compilation_context = compilation_context,
        public_hdrs = public_std_hdrs,
        header_includes = header_includes,
        action_inputs = ctx.files._builtin_hdrs + std_hdrs,
        targets_and_headers = targets_and_headers,
        deps_for_cc_file = ctx.attr._generator[GeneratedFilesDepsInfo].deps_for_cc_file,
        deps_for_rs_file = ctx.attr._generator[GeneratedFilesDepsInfo].deps_for_rs_file,
    )

bindings_for_toolchain_headers = rule(
    implementation = _bindings_for_toolchain_headers_impl,
    attrs = dict(
        bindings_attrs.items() + {
            "hdrs": attr.label(),
            "public_hdrs": attr.string_list(),
            "deps": attr.label_list(),
        }.items(),
    ),
    toolchains = [
        "//third_party/bazel_rules/rules_rust/rust:toolchain",
        "//tools/cpp:toolchain_type",
    ],
    host_fragments = ["cpp"],
    fragments = ["cpp", "google_cpp"],
)
