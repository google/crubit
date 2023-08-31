# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Generates bindings for the toolchain headers.

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "DepsForBindingsInfo",
    "RustToolchainHeadersInfo",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils.bzl",
    "bindings_attrs",
    "generate_and_compile_bindings",
)

def _has_suffix(input, suffices):
    for suffix in suffices:
        if input.short_path.endswith(suffix):
            return True
    return False

def _filter_headers_with_suffices(input_list, suffices):
    return [hdr for hdr in input_list if _has_suffix(hdr, suffices)]

def _filter_headers_without_suffices(input_list, suffices):
    return [hdr for hdr in input_list if not _has_suffix(hdr, suffices)]

def _add_prefix(strings, prefix):
    return [prefix + s for s in strings]

def _bindings_for_toolchain_headers_impl(ctx):
    std_files = ctx.attr._stl[CcInfo].compilation_context.headers.to_list() + ctx.files.hdrs
    std_and_builtin_files = depset(direct = ctx.files.hdrs + ctx.files._builtin_hdrs, transitive = [ctx.attr._stl[CcInfo].compilation_context.headers])

    prefixed_libcxx_hdrs = _add_prefix(ctx.attr.public_libcxx_hdrs, "c++/v1/")

    # The clang builtin headers also contain some libc++ headers. We consider those part of
    # the libc++ target, so we generate bindings for them.
    builtin_libcxx_files = _filter_headers_with_suffices(ctx.files._builtin_hdrs, prefixed_libcxx_hdrs)
    builtin_nonstd_files = _filter_headers_without_suffices(
        ctx.files._builtin_hdrs,
        ctx.attr.public_libcxx_hdrs,
    )

    target_args = depset(
        direct = [
            json.encode({
                "t": str(ctx.label),
                "h": [hdr.path for hdr in std_files + builtin_libcxx_files],
                "f": ["supported", "experimental"],
            }),
            json.encode({
                "t": "//:_nothing_should_depend_on_private_builtin_hdrs",
                "h": [h.path for h in builtin_nonstd_files],
            }),
        ],
    )

    public_libcxx_files = _filter_headers_with_suffices(std_files, prefixed_libcxx_hdrs)
    public_libc_files = _filter_headers_with_suffices(std_files, _add_prefix(ctx.attr.public_libc_hdrs, "v5/include/"))

    header_includes = []
    for hdr in ctx.attr.public_libcxx_hdrs + ctx.attr.public_libc_hdrs:
        header_includes.append("-include")
        header_includes.append(hdr)

    return [RustToolchainHeadersInfo(headers = std_and_builtin_files)] + generate_and_compile_bindings(
        ctx,
        ctx.attr,
        compilation_context = ctx.attr._stl[CcInfo].compilation_context,
        public_hdrs = public_libc_files + public_libcxx_files,
        header_includes = header_includes,
        action_inputs = std_and_builtin_files,
        target_args = target_args,
        extra_rs_srcs = ctx.files.extra_rs_srcs,
        deps_for_cc_file = ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_cc_file,
        deps_for_rs_file = ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_rs_file,
    )

bindings_for_toolchain_headers = rule(
    implementation = _bindings_for_toolchain_headers_impl,
    attrs = dict(
        bindings_attrs.items() + {
            "hdrs": attr.label(),
            "public_libc_hdrs": attr.string_list(),
            "public_libcxx_hdrs": attr.string_list(),
            "extra_rs_srcs": attr.label_list(allow_files = True),
            "_stl": attr.label(default = "//third_party/stl:stl"),
        }.items(),
    ),
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        "@bazel_tools//tools/cpp:toolchain_type",
    ],
    host_fragments = ["cpp"],
    fragments = ["cpp", "google_cpp"],
)
