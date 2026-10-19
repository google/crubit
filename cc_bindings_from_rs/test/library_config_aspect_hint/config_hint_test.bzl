# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Tests for config_hint aspect rule failures."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_make_analysis_test")

def _missing_hint_test_impl(ctx):
    env = analysistest.begin(ctx)
    asserts.expect_failure(env, "configures the generated C++ API, but")
    asserts.expect_failure(env, "is not attached to")
    return analysistest.end(env)

missing_hint_test = crubit_make_analysis_test(
    _missing_hint_test_impl,
    expect_failure = True,
)

def _duplicate_hint_test_impl(ctx):
    env = analysistest.begin(ctx)
    asserts.expect_failure(env, "A crate may have at most one non-empty C++ bindings configuration, but found")
    return analysistest.end(env)

duplicate_hint_test = crubit_make_analysis_test(
    _duplicate_hint_test_impl,
    expect_failure = True,
)
