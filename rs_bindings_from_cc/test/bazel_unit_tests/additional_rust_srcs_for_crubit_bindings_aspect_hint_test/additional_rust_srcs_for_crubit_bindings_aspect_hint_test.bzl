# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for extra_rs_srcs aspect hint."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli():
    additional_rust_srcs_for_crubit_bindings(
        name = "stub_additional_rust_srcs",
        srcs = [
            "stub.rs",
        ],
        tags = ["manual"],
    )
    native.cc_library(
        name = "empty_cc_lib_with_stub_additional_rust_srcs",
        hdrs = ["empty.h"],
        aspect_hints = [
            ":stub_additional_rust_srcs",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_empty_cc_lib_with_stub_additional_rust_srcs",
        dep = ":empty_cc_lib_with_stub_additional_rust_srcs",
    )

    additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test(
        name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test",
        target_under_test = ":aspect_for_empty_cc_lib_with_stub_additional_rust_srcs",
    )

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    analysis_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "CppHeaderAnalysis"][0]
    expected_flag = "--extra_rs_srcs=rs_bindings_from_cc/test/bazel_unit_tests/additional_rust_srcs_for_crubit_bindings_aspect_hint_test/stub.rs"
    asserts.true(
        env,
        expected_flag in analysis_action.argv,
        "Flag '%s' failed to be passed to rs_bindings_from_cc_driver. Actual flags: %s" % (expected_flag, analysis_action.argv),
    )
    return analysistest.end(env)

additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test = analysistest.make(_test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_impl)

def additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(name):
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli()
    native.test_suite(
        name = name,
        tests = [
            ":additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test",
        ],
    )
