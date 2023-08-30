# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for rust_bindings_from_cc_cli_flag aspect hint."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_cli_flag_aspect_hint.bzl",
    "rust_bindings_from_cc_cli_flag",
)
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _test_rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli():
    rust_bindings_from_cc_cli_flag(
        name = "disable_generate_source_location_in_doc_comment",
        flags = "--generate_source_location_in_doc_comment=False",
        tags = ["manual"],
    )
    rust_bindings_from_cc_cli_flag(
        name = "do_nothing",
        flags = "--do_nothing",
        tags = ["manual"],
    )
    native.cc_library(
        name = "cc_library_with_rs_bindings_from_cc_cli_flags",
        hdrs = ["stub_lib.h"],
        aspect_hints = [
            ":disable_generate_source_location_in_doc_comment",
            ":do_nothing",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_library_with_rs_bindings_from_cc_cli_flags",
        dep = ":cc_library_with_rs_bindings_from_cc_cli_flags",
    )

    rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_test(
        name = "rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_test",
        target_under_test = ":aspect_for_cc_library_with_rs_bindings_from_cc_cli_flags",
    )

def _test_rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    expected_cli_flags = ["--generate_source_location_in_doc_comment=False", "--do_nothing"]
    analysis_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "CppHeaderAnalysis"][0]
    for expected_cli_flag in expected_cli_flags:
        asserts.true(
            env,
            expected_cli_flag in analysis_action.argv,
            "Flag '%s' failed to be passed to rs_bindings_from_cc_driver. Actual flags: %s" % (expected_cli_flag, analysis_action.argv),
        )
    return analysistest.end(env)

rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_test = crubit_make_analysis_test(_test_rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_impl)

def rust_bindings_from_cc_cli_flag_aspect_hint_test_suite(name):
    _test_rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli()
    native.test_suite(
        name = name,
        tests = [
            ":rust_bindings_from_cc_cli_flag_aspect_hint_propagate_to_cli_test",
        ],
    )
