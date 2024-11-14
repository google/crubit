# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect hint, to be attached to a `rust_library`, specifies library configuration to be passed
to `cc_bindings_from_rust` when its Rust bindings are generated."""

load("@bazel_skylib//lib:collections.bzl", "collections")
load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")

visibility([
    "public",
])

CcBindingsFromRustLibraryConfigInfo = provider(
    doc = "The provider that specifies the configuration and values for `cc_bindings_from_rust`.",
    fields = {
        "namespace": "The top level namespace of the C++ bindings.",
        "extra_cc_hdrs": "The C++ header files to be included in addition to the generated C++ bindings.",
        "extra_cc_srcs": "The C++ source files to be included in addition to generated C++ bindings.",
    },
)

def _cc_bindings_from_rust_library_config_impl(ctx):
    return [CcBindingsFromRustLibraryConfigInfo(
        namespace = ctx.attr.namespace,
        extra_cc_hdrs = ctx.attr.extra_cc_hdrs,
        extra_cc_srcs = ctx.attr.extra_cc_srcs,
    )]

cc_bindings_from_rust_library_config = rule(
    attrs = {
        "extra_cc_srcs": attr.label_list(
            doc = "The C++ source files to be incldued in addition to generated C++ bindings.",
            allow_files = [".cc"],
            mandatory = False,
        ),
        "extra_cc_hdrs": attr.label_list(
            doc = "The C++ header files to be incldued in addition to generated C++ bindings.",
            allow_files = [".h"],
            mandatory = False,
        ),
        "namespace": attr.string(
            doc = "The top level namespace of the C++ bindings.",
        ),
    },
    implementation = _cc_bindings_from_rust_library_config_impl,
    doc = """
Defines an aspect hint that is used to pass configuration to the `cc_bindings_from_rust` tool,
which affects the tool behavior when generating the C++ binding for the Rust target.
""",
)

def crate_name_to_library_config(aspect_ctx):
    """Returns the configuration for `cc_bindings_from_rust`.

    Args:
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A map from crate name to the configuration for `cc_bindings_from_rust`.
    """
    crate_config_map = {}
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if CcBindingsFromRustLibraryConfigInfo in hint:
            crate_config_map["self"] = hint[CcBindingsFromRustLibraryConfigInfo]
    for dep in aspect_ctx.rule.attr.deps:
        if CcBindingsFromRustInfo in dep:
            rust_info = dep[CcBindingsFromRustInfo]
            if rust_info.configuration:
                crate_config_map[dep.label.name] = rust_info.configuration
    return crate_config_map

def get_additional_cc_hdrs_and_srcs(aspect_ctx):
    """Returns any additional C++ headers and sources that should be compiled with the generated bindings.

    Args:
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A list of `File` and its module paths as specified by the `extra_rs_srcs`.
    """
    additional_cc_hdrs = []
    additional_cc_srcs = []
    for hint in aspect_ctx.rule.attr.aspect_hints:
        if CcBindingsFromRustLibraryConfigInfo in hint:
            for target in hint[CcBindingsFromRustLibraryConfigInfo].extra_cc_hdrs:
                additional_cc_hdrs.extend(target.files.to_list())
            for target in hint[CcBindingsFromRustLibraryConfigInfo].extra_cc_srcs:
                additional_cc_srcs.extend(target.files.to_list())
    return (collections.uniq(additional_cc_hdrs), collections.uniq(additional_cc_srcs))
