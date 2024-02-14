# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that generates bindings source files for a given C++ library."""

load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_flavor_transition",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)

def _generate_bindings_impl(ctx):
    cc_library = ctx.attr.cc_library[0]
    if not GeneratedBindingsInfo in cc_library:
        fail("Bindings were not generated for the given cc_library.")
    bindings = cc_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        cc_file = [bindings.cc_file],
        rust_file = [bindings.rust_file],
        namespaces_file = [bindings.namespaces_file],
    )

_generate_bindings = rule(
    attrs = {
        "cc_library": attr.label(
            providers = [CcInfo],
            aspects = [rust_bindings_from_cc_aspect],
            cfg = crubit_flavor_transition,
        ),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
    implementation = _generate_bindings_impl,
)

def golden_test(
        name,
        cc_library,
        tags = None,
        basename = None,
        golden_cc = None,
        golden_rs = None,
        golden_namespaces = None):
    """Generates a golden test for `cc_library`.

    Args:
        name: The name of the golden test.
        cc_library: The C++ library whose outputs should be checked.
        tags: The test tags.
        basename: The name to use for generated files.
        golden_cc: The generated C++ source code for the bindings.
        golden_rs: The generated Rust source code for the bindings.
        golden_namespaces: The generated namespaces JSON file for the bindings.

    """
    if not basename:
        basename = name
    if not tags:
        tags = []
    tags.append("crubit_golden_test")

    bindings_name = basename + ".generated_bindings"

    _generate_bindings(
        name = bindings_name,
        cc_library = cc_library,
    )
    args = []
    data = ["//rs_bindings_from_cc/test/golden:LICENSE_HEADER"]
    owned_files = []
    if golden_cc:
        new_cc = basename + ".cc_file"
        native.filegroup(
            name = new_cc,
            srcs = [bindings_name],
            output_group = "cc_file",
        )
        args += [
            "$(location %s)" % golden_cc,
            "$(location %s)" % new_cc,
        ]
        data += [
            golden_cc,
            new_cc,
        ]
        owned_files.append(golden_cc)

    if golden_rs:
        new_rs = basename + ".rs_file"
        native.filegroup(
            name = new_rs,
            srcs = [bindings_name],
            output_group = "rust_file",
        )
        args += [
            "$(location %s)" % golden_rs,
            "$(location %s)" % new_rs,
        ]
        data += [
            golden_rs,
            new_rs,
        ]
        owned_files.append(golden_rs)

    if golden_namespaces:
        new_namespaces = basename + ".namespaces_file"
        native.filegroup(
            name = new_namespaces,
            srcs = [bindings_name],
            output_group = "namespaces_file",
        )
        args += [
            "$(location %s)" % golden_namespaces,
            "$(location %s)" % new_namespaces,
        ]
        data += [
            golden_namespaces,
            new_namespaces,
        ]
        owned_files.append(golden_namespaces)

    native.sh_test(
        name = name,
        srcs = ["//rs_bindings_from_cc/test/golden:test.sh"],
        args = args,
        data = data,
        tags = tags,
    )
    native.filegroup(
        name = basename + ".build_cleaner_optout",
        srcs = owned_files,
        tags = ["ignore_srcs"],
        visibility = ["//visibility:private"],
    )
