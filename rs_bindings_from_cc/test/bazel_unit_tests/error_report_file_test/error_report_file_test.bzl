# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Unit tests for `GeneratedBindingsInfo.error_report_file`."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load("//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl", "attach_aspect")

def _error_report_enabled_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    error_report_file = target_under_test[GeneratedBindingsInfo].error_report_file
    asserts.true(env, error_report_file != None, "expected an error report file")
    if error_report_file != None:
        asserts.true(
            env,
            error_report_file.basename.endswith("_rust_api_error_report.json"),
            "unexpected error report file: " + error_report_file.basename,
        )

    return analysistest.end(env)

_error_report_enabled_test = crubit_make_analysis_test(
    _error_report_enabled_test_impl,
    config_settings = {
        "//rs_bindings_from_cc/bazel_support:generate_error_report": True,
    },
)

def _error_report_disabled_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    asserts.equals(env, None, target_under_test[GeneratedBindingsInfo].error_report_file)

    return analysistest.end(env)

_error_report_disabled_test = crubit_make_analysis_test(
    _error_report_disabled_test_impl,
    config_settings = {
        "//rs_bindings_from_cc/bazel_support:generate_error_report": False,
    },
)

def error_report_file_test(name):
    """Sets up the error_report_file test suite.

    Args:
      name: name of the test suite
    """
    cc_library(
        name = "lib",
        hdrs = ["lib.h"],
        aspect_hints = ["//features:supported"],
    )

    # Runs Crubit's aspect on `:lib`, and re-exports its `GeneratedBindingsInfo` so that the tests
    # below can read it from `:lib_bindings`.
    attach_aspect(name = "lib_bindings", dep = ":lib")

    _error_report_enabled_test(
        name = "error_report_enabled_test",
        target_under_test = ":lib_bindings",
    )

    _error_report_disabled_test(
        name = "error_report_disabled_test",
        target_under_test = ":lib_bindings",
    )

    native.test_suite(
        name = name,
        tests = [
            ":error_report_disabled_test",
            ":error_report_enabled_test",
        ],
    )
