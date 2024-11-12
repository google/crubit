# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""The aspect hint, to be attached to a `rust_library`, specifies library configuration to be passed
to `cc_bindings_from_rust` when its Rust bindings are generated."""

load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")

visibility([
    "public",
])

CcBindingsFromRustLibraryConfigInfo = provider(
    doc = "The provider that specifies the configuration and values for `cc_bindings_from_rust`.",
    fields = {
        "namespace": "The top level namespace of the C++ bindings.",
    },
)

def _cc_bindings_from_rust_library_config_impl(ctx):
    return [CcBindingsFromRustLibraryConfigInfo(
        namespace = ctx.attr.namespace,
    )]

cc_bindings_from_rust_library_config = rule(
    attrs = {
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
