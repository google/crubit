# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for rust_bindings_from_cc_aspect."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _is_std(t):
    return str(t) in ["//rs_bindings_from_cc:cc_std", "//:_builtin_hdrs"]

def _get_targets_and_headers(tut):
    return [
        x
        for x in [
            json.decode(tah)
            for tah in tut[RustBindingsFromCcInfo].targets_and_headers.to_list()
        ]
        if not _is_std(x["t"])
    ]

def _lib_has_toolchain_targets_and_headers_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    targets_and_headers = [
        json.decode(tah)
        for tah in target_under_test[RustBindingsFromCcInfo].targets_and_headers.to_list()
    ]

    asserts.equals(env, 3, len(targets_and_headers))
    asserts.equals(
        env,
        targets_and_headers[0]["t"],
        "//rs_bindings_from_cc:cc_std",
    )
    asserts.equals(
        env,
        targets_and_headers[1]["t"],
        "//:_nothing_should_depend_on_private_builtin_hdrs",
    )
    asserts.equals(
        env,
        targets_and_headers[2]["t"],
        "//rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets:empty",
    )

    return analysistest.end(env)

lib_has_toolchain_targets_and_headers_test = analysistest.make(
    _lib_has_toolchain_targets_and_headers_test_impl,
)

def _test_lib_has_toolchain_targets_and_headers():
    native.cc_library(name = "empty")
    attach_aspect(name = "empty_with_aspect", dep = ":empty")
    lib_has_toolchain_targets_and_headers_test(
        name = "lib_has_toolchain_targets_and_headers_test",
        target_under_test = ":empty_with_aspect",
    )

def _targets_and_headers_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    targets_and_headers = _get_targets_and_headers(target_under_test)

    asserts.equals(env, 2, len(targets_and_headers))
    asserts.equals(
        env,
        targets_and_headers[1]["t"],
        "//rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets:mylib",
    )
    asserts.equals(
        env,
        targets_and_headers[1]["h"],
        ["rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/lib.h"],
    )

    return analysistest.end(env)

targets_and_headers_test = analysistest.make(_targets_and_headers_test_impl)

def _test_targets_and_headers():
    native.cc_library(name = "mylib", hdrs = ["lib.h"])
    attach_aspect(name = "mylib_with_aspect", dep = ":mylib")

    targets_and_headers_test(
        name = "targets_and_headers_test",
        target_under_test = ":mylib_with_aspect",
    )

def _targets_and_headers_propagate_with_cc_info_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    targets_and_headers = _get_targets_and_headers(target_under_test)

    asserts.equals(env, 4, len(targets_and_headers))
    asserts.equals(
        env,
        targets_and_headers[0]["t"],
        "//:_nothing_should_depend_on_private_builtin_hdrs",
    )
    asserts.equals(
        env,
        targets_and_headers[1]["t"],
        "//rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets:bottom",
    )
    asserts.equals(
        env,
        targets_and_headers[1]["h"],
        ["rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/lib.h"],
    )

    asserts.equals(
        env,
        targets_and_headers[2]["t"],
        "//rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets:middle",
    )
    asserts.true(
        env,
        targets_and_headers[2]["h"][0].endswith(
            "rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/middle.empty_source_no_public_headers.h",
        ),
    )

    asserts.equals(
        env,
        targets_and_headers[3]["t"],
        "//rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets:top",
    )
    asserts.equals(
        env,
        targets_and_headers[3]["h"],
        ["rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/top.h"],
    )

    return analysistest.end(env)

targets_and_headers_propagate_with_cc_info_test = analysistest.make(
    _targets_and_headers_propagate_with_cc_info_test_impl,
)

def _test_targets_and_headers_propagate_with_cc_infos():
    native.cc_library(name = "bottom", hdrs = ["lib.h"])
    native.cc_library(name = "middle", deps = [":bottom"])
    native.cc_library(name = "top", hdrs = ["top.h"], deps = [":middle"])
    attach_aspect(name = "top_with_aspect", dep = ":top")

    targets_and_headers_propagate_with_cc_info_test(
        name = "targets_and_headers_propagate_with_cc_info_test",
        target_under_test = ":top_with_aspect",
    )

def _textual_hdrs_not_in_targets_and_hdrs_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    targets_and_headers = _get_targets_and_headers(target_under_test)

    # Check that none of the textual headers made it into the targets_and_headers provider.
    asserts.equals(env, 2, len(targets_and_headers))
    asserts.equals(
        env,
        targets_and_headers[1]["h"],
        ["rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/nontextual.h"],
    )

    return analysistest.end(env)

textual_hdrs_not_in_targets_and_hdrs_test = analysistest.make(
    _textual_hdrs_not_in_targets_and_hdrs_impl,
)

def _toolchain_headers_in_header_analysis_action_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    analysis_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "CppHeaderAnalysis"][0]
    inputs = analysis_action.inputs.to_list()
    inttypes = [i.path for i in inputs if "inttypes.h" in i.path]
    asserts.equals(
        env,
        ["nowhere/llvm/toolchain/include/c++/v1/inttypes.h", "//nowhere/libcinclude/inttypes.h", "third_party/llvm/llvm-project/clang/lib/Headers/inttypes.h"],
        inttypes,
    )

    return analysistest.end(env)

toolchain_headers_in_header_analysis_action_test = analysistest.make(
    _toolchain_headers_in_header_analysis_action_test_impl,
    config_settings = {
        "//command_line_option:crosstool_top": "//nowhere/llvm:everything",
    },
)

def _test_textual_hdrs_not_in_targets_and_hdrs():
    native.cc_library(
        name = "textual",
        hdrs = [
            "nontextual.h",
            "textual_in_hdrs.inc",
        ],
        srcs = ["textual_in_srcs.inc"],
        textual_hdrs = ["textual1.inc", "textual2.h"],
    )
    attach_aspect(name = "textual_with_aspect", dep = ":textual")

    textual_hdrs_not_in_targets_and_hdrs_test(
        name = "textual_hdrs_not_in_targets_and_hdrs_test",
        target_under_test = ":textual_with_aspect",
    )

def _test_toolchain_headers_in_header_analysis_action():
    native.cc_library(
        name = "somelib",
        hdrs = ["someheader.h"],
    )
    attach_aspect(name = "somelib_with_aspect", dep = ":somelib")

    toolchain_headers_in_header_analysis_action_test(
        name = "toolchain_headers_in_header_analysis_action_test",
        target_under_test = ":somelib_with_aspect",
    )

def _generated_headers_specified_with_full_path_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    targets_and_headers = _get_targets_and_headers(target_under_test)

    asserts.equals(env, 2, len(targets_and_headers))
    header_path = targets_and_headers[1]["h"][0]
    asserts.true(
        env,
        header_path
            .endswith("rs_bindings_from_cc/test/bazel_unit_tests/headers_and_targets/generated.h"),
    )
    asserts.true(
        env,
        header_path.startswith("bazel-out"),
    )

    return analysistest.end(env)

generated_headers_specified_with_full_path_test = analysistest.make(
    _generated_headers_specified_with_full_path_impl,
)

def _test_generated_headers_specified_with_full_path():
    native.genrule(
        name = "generate_header",
        outs = ["generated.h"],
        cmd = "touch $@",
    )
    native.cc_library(
        name = "use_generated",
        hdrs = [
            "generated.h",
        ],
    )
    attach_aspect(name = "generated_header_with_aspect", dep = ":use_generated")

    generated_headers_specified_with_full_path_test(
        name = "generated_headers_specified_with_full_path_test",
        target_under_test = ":generated_header_with_aspect",
    )

def headers_and_targets_test(name):
    """Sets up rust_bindings_from_cc_aspect test suite.

    Args:
      name: name of the test suite"""
    _test_targets_and_headers()
    _test_targets_and_headers_propagate_with_cc_infos()
    _test_textual_hdrs_not_in_targets_and_hdrs()
    _test_lib_has_toolchain_targets_and_headers()
    _test_toolchain_headers_in_header_analysis_action()
    _test_generated_headers_specified_with_full_path()

    native.test_suite(
        name = name,
        tests = [
            ":targets_and_headers_test",
            ":targets_and_headers_propagate_with_cc_info_test",
            ":textual_hdrs_not_in_targets_and_hdrs_test",
            ":lib_has_toolchain_targets_and_headers_test",
            ":toolchain_headers_in_header_analysis_action_test",
            ":generated_headers_specified_with_full_path_test",
        ],
    )
