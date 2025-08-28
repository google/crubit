# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Generates bindings for the toolchain headers."""

load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "AdditionalRustSrcsProviderInfo",
    "DepsForBindingsInfo",
    "RustToolchainHeadersInfo",
)
load(
    "@@//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils.bzl",
    "bindings_attrs",
    "generate_and_compile_bindings",
)

def _has_suffix(input, suffixes):
    for suffix in suffixes:
        if input.short_path.endswith(suffix):
            return True
    return False

def _filter_headers_with_suffixes(input_list, suffixes):
    return [hdr for hdr in input_list if _has_suffix(hdr, suffixes)]

def _filter_headers_without_suffixes(input_list, suffixes):
    return [hdr for hdr in input_list if not _has_suffix(hdr, suffixes)]

def _add_prefix(strings, prefix):
    return [prefix + s for s in strings]

def _bindings_for_toolchain_headers_impl(ctx):
    toolchain = ctx.toolchains["@@//rs_bindings_from_cc/bazel_support:toolchain_type"].rs_bindings_from_cc_toolchain_info
    builtin_headers = toolchain.builtin_headers

    stl_headers = toolchain.stl_headers + ctx.files.extra_hdrs

    std_files = ctx.attr._stl[CcInfo].compilation_context.headers.to_list() + stl_headers
    std_and_builtin_files = depset(direct = stl_headers + builtin_headers, transitive = [ctx.attr._stl[CcInfo].compilation_context.headers])

    prefixed_libcxx_hdrs = _add_prefix(ctx.attr.public_libcxx_hdrs, "c++/v1/")

    # The clang builtin headers also contain some libc++ headers. We consider those part of
    # the libc++ target, so we generate bindings for them.
    builtin_libcxx_files = _filter_headers_with_suffixes(builtin_headers, prefixed_libcxx_hdrs)
    builtin_nonstd_files = _filter_headers_without_suffixes(
        builtin_headers,
        ctx.attr.public_libcxx_hdrs,
    )

    target_args = depset(
        direct = [
            json.encode({
                "t": str(ctx.label),
                "h": [hdr.path for hdr in std_files + builtin_libcxx_files],
                "f": ["all"],
            }),
            json.encode({
                "t": "//:_nothing_should_depend_on_private_builtin_hdrs",
                "h": [h.path for h in builtin_nonstd_files],
            }),
        ],
    )

    public_libcxx_files = _filter_headers_with_suffixes(std_files, prefixed_libcxx_hdrs)
    public_libc_files = _filter_headers_with_suffixes(std_files, _add_prefix(ctx.attr.public_libc_hdrs, "v5/include/"))

    header_includes = []
    for hdr in ctx.attr.public_libcxx_hdrs + ctx.attr.public_libc_hdrs:
        header_includes.append("-include")
        header_includes.append(hdr)
    extra_rs_srcs = []
    extra_rs_deps = []
    for target in ctx.attr.extra_rs_srcs:
        if AdditionalRustSrcsProviderInfo in target:
            for src in target[AdditionalRustSrcsProviderInfo].srcs:
                extra_rs_srcs.extend([(f, target[AdditionalRustSrcsProviderInfo].namespace_path) for f in src.files.to_list()])
            extra_rs_deps.extend(target[AdditionalRustSrcsProviderInfo].deps)
            if target[AdditionalRustSrcsProviderInfo].cc_deps:
                fail("toolchain_headers do not accept additional Rust cc_deps")
        else:
            extra_rs_srcs.extend([(f, "") for f in target.files.to_list()])
    return [RustToolchainHeadersInfo(headers = std_and_builtin_files)] + generate_and_compile_bindings(
        ctx,
        ctx.attr,
        compilation_context = ctx.attr._stl[CcInfo].compilation_context,
        public_hdrs = public_libc_files + public_libcxx_files + ctx.files.extra_hdrs,
        header_includes = header_includes,
        action_inputs = std_and_builtin_files,
        target_args = target_args,
        extra_rs_srcs = extra_rs_srcs,
        deps_for_cc_file = ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_cc_file,
        deps_for_rs_file = depset(extra_rs_deps + ctx.attr._deps_for_bindings[DepsForBindingsInfo].deps_for_rs_file),
    )

bindings_for_toolchain_headers = rule(
    implementation = _bindings_for_toolchain_headers_impl,
    attrs = bindings_attrs | {
        # Additional internal headers that are not part of the standard library. These headers will
        # receive bindings which are exposed along with the standard library bindings.
        # Everything inside these under should be hidden within namespace `crubit_cc_std_internal`.
        "extra_hdrs": attr.label_list(default = []),
        "public_libc_hdrs": attr.string_list(),
        "public_libcxx_hdrs": attr.string_list(),
        "extra_rs_srcs": attr.label_list(allow_files = True),
        # This isn't read directly by _bindings_for_toolchain_headers_impl, but is read by
        # rules_rust when compiling the generated and additional Rust source code.
        "crate_features": attr.string_list(),
        "_stl": attr.label(default = "//third_party/stl:stl"),
        # TODO: b/421934470 - Fix uses of exec groups and re-enable AEG
        "_use_auto_exec_groups": attr.bool(default = False),
    },
    toolchains = [
        "@rules_rust//rust:toolchain_type",
        "@bazel_tools//tools/cpp:toolchain_type",
        "@@//rs_bindings_from_cc/bazel_support:toolchain_type",
    ],
    fragments = ["cpp", "google_cpp"],
)
