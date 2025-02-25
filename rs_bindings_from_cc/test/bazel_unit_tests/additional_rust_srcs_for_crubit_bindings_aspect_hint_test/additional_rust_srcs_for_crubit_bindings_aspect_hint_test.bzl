# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for extra_rs_srcs aspect hint."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/bazel_support:providers.bzl", "RustBindingsFromCcInfo")
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

additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test = crubit_make_analysis_test(_test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_impl)

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers():
    additional_rust_srcs_for_crubit_bindings(
        name = "some_additional_rust_srcs",
        srcs = [
            "stub.rs",
        ],
        tags = ["manual"],
    )
    native.cc_library(
        name = "cc_lib_with_some_additional_rust_srcs",
        aspect_hints = [
            ":some_additional_rust_srcs",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_lib_with_some_additional_rust_srcs",
        dep = ":cc_lib_with_some_additional_rust_srcs",
    )

    test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_test(
        name = "test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_test",
        target_under_test = ":aspect_for_cc_lib_with_some_additional_rust_srcs",
    )

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    asserts.true(env, RustBindingsFromCcInfo in target_under_test, "exptected target to have RustBindingFromCcInfo")
    bindings_info = target_under_test[RustBindingsFromCcInfo]
    asserts.true(env, bindings_info.dep_variant_info != None, "expected target to have DepVariantInfo")
    return analysistest.end(env)

test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_test = crubit_make_analysis_test(_test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_impl)

def additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(name):
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli()
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers()
    native.test_suite(
        name = name,
        tests = [
            ":additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test",
            ":test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_test",
        ],
    )
