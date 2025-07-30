# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for rust_bindings_from_cc_aspect."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)
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
    for std_pattern in [
        "crubit/support/cc_std:cc_std",
        "//:_builtin_hdrs",
    ]:
        if std_pattern in str(t):
            return True
    return False

def _get_target_args(tut):
    return [
        x
        for x in [
            json.decode(args)
            for args in tut[RustBindingsFromCcInfo].target_args.to_list()
        ]
        if not _is_std(x["t"])
    ]

def _lib_has_toolchain_targets_and_headers_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = [
        json.decode(args)
        for args in target_under_test[RustBindingsFromCcInfo].target_args.to_list()
    ]

    asserts.equals(env, 3, len(target_args))
    asserts.true(
        env,
        "crubit/support/cc_std:cc_std" in target_args[0]["t"],
    )
    asserts.equals(
        env,
        "//:_nothing_should_depend_on_private_builtin_hdrs",
        target_args[1]["t"],
    )
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:empty",
        target_args[2]["t"],
    )

    return analysistest.end(env)

lib_has_toolchain_targets_and_headers_test = crubit_make_analysis_test(
    _lib_has_toolchain_targets_and_headers_test_impl,
)

def _test_lib_has_toolchain_targets_and_headers():
    cc_library(name = "empty")
    attach_aspect(name = "empty_with_aspect", dep = ":empty")
    lib_has_toolchain_targets_and_headers_test(
        name = "lib_has_toolchain_targets_and_headers_test",
        target_under_test = ":empty_with_aspect",
    )

def _targets_and_headers_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    asserts.equals(env, 2, len(target_args))
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:mylib",
        target_args[1]["t"],
    )
    asserts.equals(
        env,
        ["rs_bindings_from_cc/test/bazel_unit_tests/target_args/lib.h"],
        target_args[1]["h"],
    )

    return analysistest.end(env)

targets_and_headers_test = crubit_make_analysis_test(_targets_and_headers_test_impl)

def _test_targets_and_headers():
    cc_library(
        name = "mylib",
        hdrs = ["lib.h"],
        aspect_hints = ["//features/internal:testonly_experimental"],
    )
    attach_aspect(name = "mylib_with_aspect", dep = ":mylib")

    targets_and_headers_test(
        name = "targets_and_headers_test",
        target_under_test = ":mylib_with_aspect",
    )

def _targets_and_headers_propagate_with_cc_info_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    asserts.equals(env, 5, len(target_args))
    asserts.equals(
        env,
        "//:_nothing_should_depend_on_private_builtin_hdrs",
        target_args[0]["t"],
    )

    # Even though the dependency doesn't have public headers, Crubit should still get metadata.
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:bottommest_no_crubit_aspect_hint",
        target_args[1]["t"],
    )
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:bottom",
        target_args[2]["t"],
    )
    asserts.equals(
        env,
        ["rs_bindings_from_cc/test/bazel_unit_tests/target_args/lib.h"],
        target_args[2]["h"],
    )

    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:middle",
        target_args[3]["t"],
    )
    asserts.true(
        env,
        target_args[3]["h"][0].endswith(
            "rs_bindings_from_cc/test/bazel_unit_tests/target_args/middle.empty_source_no_public_headers.h",
        ),
    )

    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:top",
        target_args[4]["t"],
    )
    asserts.equals(
        env,
        ["rs_bindings_from_cc/test/bazel_unit_tests/target_args/top.h"],
        target_args[4]["h"],
    )

    return analysistest.end(env)

targets_and_headers_propagate_with_cc_info_test = crubit_make_analysis_test(
    _targets_and_headers_propagate_with_cc_info_test_impl,
)

def _test_targets_and_headers_propagate_with_cc_infos():
    cc_library(name = "bottommest_no_crubit_aspect_hint")
    cc_library(name = "bottom", hdrs = ["lib.h"], deps = [":bottommest_no_crubit_aspect_hint"], aspect_hints = ["//features/internal:testonly_experimental"])
    cc_library(name = "middle", deps = [":bottom"], aspect_hints = ["//features/internal:testonly_experimental"])
    cc_library(name = "top", hdrs = ["top.h"], deps = [":middle"], aspect_hints = ["//features/internal:testonly_experimental"])
    attach_aspect(name = "top_with_aspect", dep = ":top")

    targets_and_headers_propagate_with_cc_info_test(
        name = "targets_and_headers_propagate_with_cc_info_test",
        target_under_test = ":top_with_aspect",
    )

def _textual_hdrs_not_in_targets_and_hdrs_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    # Check that none of the textual headers made it into the target_args provider.
    asserts.equals(env, 2, len(target_args))
    asserts.equals(
        env,
        ["rs_bindings_from_cc/test/bazel_unit_tests/target_args/nontextual.h"],
        target_args[1]["h"],
    )

    return analysistest.end(env)

textual_hdrs_not_in_targets_and_hdrs_test = crubit_make_analysis_test(
    _textual_hdrs_not_in_targets_and_hdrs_impl,
)

def _toolchain_headers_in_header_analysis_action_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    analysis_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "CppHeaderAnalysis"][0]
    inputs = analysis_action.inputs.to_list()
    inttypes = [i.path for i in inputs if "inttypes.h" in i.path]
    asserts.true(
        env,
        any([path in [
            "nowhere/llvm/src/libcxx/include/inttypes.h",
        ] for path in inttypes]),
        "inttypes: %s" % inttypes,
    )
    asserts.true(
        env,
        any([path in [
            "//nowhere/libc_x86include/inttypes.h",
            "//nowhere/libc_arminclude/inttypes.h",
        ] for path in inttypes]),
        "inttypes: %s" % inttypes,
    )
    asserts.true(
        env,
        "third_party/llvm/llvm-project/clang/lib/Headers/inttypes.h" in inttypes,
        "inttypes: %s" % inttypes,
    )

    return analysistest.end(env)

toolchain_headers_in_header_analysis_action_test = crubit_make_analysis_test(
    _toolchain_headers_in_header_analysis_action_test_impl,
)

def _test_textual_hdrs_not_in_targets_and_hdrs():
    cc_library(
        name = "textual",
        hdrs = [
            "nontextual.h",
            "textual_in_hdrs.inc",
        ],
        srcs = ["textual_in_srcs.inc"],
        aspect_hints = ["//features/internal:testonly_experimental"],
        textual_hdrs = ["textual1.inc", "textual2.h"],
    )
    attach_aspect(name = "textual_with_aspect", dep = ":textual")

    textual_hdrs_not_in_targets_and_hdrs_test(
        name = "textual_hdrs_not_in_targets_and_hdrs_test",
        target_under_test = ":textual_with_aspect",
    )

def _test_toolchain_headers_in_header_analysis_action():
    cc_library(
        name = "somelib",
        hdrs = ["someheader.h"],
        aspect_hints = ["//features/internal:testonly_experimental"],
    )
    attach_aspect(name = "somelib_with_aspect", dep = ":somelib")

    toolchain_headers_in_header_analysis_action_test(
        name = "toolchain_headers_in_header_analysis_action_test",
        target_under_test = ":somelib_with_aspect",
    )

def _generated_headers_specified_with_full_path_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    asserts.equals(env, 2, len(target_args))
    header_path = target_args[1]["h"][0]
    asserts.true(
        env,
        header_path
            .endswith("rs_bindings_from_cc/test/bazel_unit_tests/target_args/generated.h"),
    )
    asserts.true(
        env,
        header_path.startswith("bazel-out"),
    )

    return analysistest.end(env)

generated_headers_specified_with_full_path_test = crubit_make_analysis_test(
    _generated_headers_specified_with_full_path_impl,
)

def _test_generated_headers_specified_with_full_path():
    native.genrule(
        name = "generate_header",
        outs = ["generated.h"],
        cmd = "touch $@",
    )
    cc_library(
        name = "use_generated",
        hdrs = [
            "generated.h",
        ],
        aspect_hints = ["//features/internal:testonly_experimental"],
    )
    attach_aspect(name = "generated_header_with_aspect", dep = ":use_generated")

    generated_headers_specified_with_full_path_test(
        name = "generated_headers_specified_with_full_path_test",
        target_under_test = ":generated_header_with_aspect",
    )

def _target_features_empty_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    asserts.equals(env, 2, len(target_args))
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:mylib_empty_features",
        target_args[1]["t"],
    )
    asserts.equals(
        env,
        None,
        target_args[1].get("f"),
    )

    return analysistest.end(env)

target_features_empty_test = crubit_make_analysis_test(_target_features_empty_test_impl)

def _test_target_features_empty():
    cc_library(name = "mylib_empty_features", hdrs = ["lib.h"])
    attach_aspect(name = "mylib_empty_features_with_aspect", dep = ":mylib_empty_features")

    target_features_empty_test(
        name = "target_features_empty_test",
        target_under_test = ":mylib_empty_features_with_aspect",
    )

def _target_features_nonempty_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    target_args = _get_target_args(target_under_test)

    asserts.equals(env, 2, len(target_args))
    asserts.equals(
        env,
        "//rs_bindings_from_cc/test/bazel_unit_tests/target_args:mylib_nonempty_features",
        target_args[1]["t"],
    )
    asserts.equals(
        env,
        ["experimental", "supported"],
        target_args[1]["f"],
    )

    return analysistest.end(env)

target_features_nonempty_test = crubit_make_analysis_test(_target_features_nonempty_test_impl)

def _test_target_features_nonempty():
    cc_library(name = "mylib_nonempty_features", hdrs = ["lib.h"], aspect_hints = [
        "//features/internal:testonly_supported",
        "//features/internal:testonly_experimental",  # merged in as well
    ])
    attach_aspect(name = "mylib_nonempty_features_with_aspect", dep = ":mylib_nonempty_features")

    target_features_nonempty_test(
        name = "target_features_nonempty_test",
        target_under_test = ":mylib_nonempty_features_with_aspect",
    )

def target_args_test(name):
    """Sets up rust_bindings_from_cc_aspect test suite.

    Args:
      name: name of the test suite"""
    _test_targets_and_headers()
    _test_targets_and_headers_propagate_with_cc_infos()
    _test_textual_hdrs_not_in_targets_and_hdrs()
    _test_lib_has_toolchain_targets_and_headers()
    _test_toolchain_headers_in_header_analysis_action()
    _test_generated_headers_specified_with_full_path()
    _test_target_features_empty()
    _test_target_features_nonempty()

    native.test_suite(
        name = name,
        tests = [
            ":targets_and_headers_test",
            ":targets_and_headers_propagate_with_cc_info_test",
            ":textual_hdrs_not_in_targets_and_hdrs_test",
            ":lib_has_toolchain_targets_and_headers_test",
            ":toolchain_headers_in_header_analysis_action_test",
            ":generated_headers_specified_with_full_path_test",
            ":target_features_empty_test",
            ":target_features_nonempty_test",
        ],
    )
