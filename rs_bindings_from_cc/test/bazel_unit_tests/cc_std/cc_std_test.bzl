# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for rust_bindings_from_cc_aspect."""

load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_flavor_transition",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)
load("@bazel_tools//tools/cpp:toolchain_utils.bzl", "find_cpp_toolchain")

OutputsInfo = provider(
    doc = """A helper provider for determining the command line. """,
    fields = {
        "args": "Args object",
        "empty_header_file": "Empty header file that Crubit creates if no public headers exist.",
    },
)

def _get_args_aspect_impl(target, ctx):
    bindings_action = None

    # Find CppHeaderAnalysis action
    for action in target.actions:
        if action.mnemonic == "CppHeaderAnalysis":
            bindings_action = action
            break

    # Mirror Crubit's behavior of creating an empty header file if no public headers exist.
    # We need this file because it is referenced in the command line.
    empty_header_file = ctx.actions.declare_file(
        ctx.label.name + ".empty_source_no_public_headers.h",
    )
    ctx.actions.write(
        empty_header_file,
        "// File intentionally left empty, its purpose is to satisfy rules_rust APIs.",
    )

    args = bindings_action.args[0]
    return [OutputsInfo(args = args, empty_header_file = empty_header_file)]

get_args_aspect = aspect(
    implementation = _get_args_aspect_impl,
    attr_aspects = ["deps"],
    requires = [rust_bindings_from_cc_aspect],
)

def _cc_std_test_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name + "_test.sh")
    dep = ctx.attr.dep

    cc_toolchain = find_cpp_toolchain(ctx)
    bindings_toolchain = ctx.toolchains["//rs_bindings_from_cc/bazel_support:toolchain_type"].rs_bindings_from_cc_toolchain_info

    ctx.actions.run(
        inputs = depset(
            direct = [
                dep[RustBindingsFromCcInfo].dep_variant_info.crate_info.output,
                dep[OutputsInfo].empty_header_file,
            ] + bindings_toolchain.stl_headers,
            transitive = [
                # For the `clang` binary.
                cc_toolchain.all_files,
                ctx.attr._stl_headers.files,
                ctx.attr._llvm_builtin_headers.files,
            ],
        ),
        arguments = [out.path, dep[OutputsInfo].args],
        executable = ctx.executable._include_directives_checker,
        env = {
            "CC": ctx.var["CC"],
        },
        outputs = [out],
    )
    return [DefaultInfo(executable = out)]

cc_std_test = rule(
    implementation = _cc_std_test_impl,
    # TODO(mboehme): Consider removing this transition so that we can run the test both for
    # the stable and the unstable configuration.
    cfg = crubit_flavor_transition,
    attrs = {
        "dep": attr.label(
            aspects = [rust_bindings_from_cc_aspect, get_args_aspect],
        ),
        "_include_directives_checker": attr.label(
            default = "//rs_bindings_from_cc/test/bazel_unit_tests/cc_std:check_include_directives",
            executable = True,
            cfg = "exec",
        ),
        "_stl_headers": attr.label(
            default = "//third_party/stl:stl_headers",
        ),
        "_llvm_builtin_headers": attr.label(
            default = "@llvm-project//clang:builtin_headers_files",
        ),
        # TODO: b/421934470 - Fix uses of exec groups and re-enable AEG
        "_use_auto_exec_groups": attr.bool(default = False),
    },
    toolchains = [
        "@bazel_tools//tools/cpp:toolchain_type",
        "//rs_bindings_from_cc/bazel_support:toolchain_type",
    ],
    test = True,
)
