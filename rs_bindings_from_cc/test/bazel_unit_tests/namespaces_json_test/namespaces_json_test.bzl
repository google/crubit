# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for the _namespace.json output file."""

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
    "attach_aspect",
)

def _action_outputs_analysis_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    asserts.true(env, target_under_test[RustBindingsFromCcInfo].namespaces != None)

    return analysistest.end(env)

action_outputs_analysis_test = crubit_make_analysis_test(
    _action_outputs_analysis_test_impl,
)

def _targets_for_namespaces():
    native.cc_library(
        name = "lib",
        hdrs = [
            "lib.h",
        ],
    )

    attach_aspect(name = "namespaces_json_outputs", dep = ":lib")

    action_outputs_analysis_test(
        name = "namespaces_json_outputs_test",
        target_under_test = ":namespaces_json_outputs",
    )

def namespaces_json_test(name):
    """Sets up namespaces_json test suite.

    Args:
      name: name of the test suite"""
    _targets_for_namespaces()

    native.test_suite(
        name = name,
        tests = [
            ":namespaces_json_outputs_test",
        ],
    )
