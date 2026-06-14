# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Tests for `crubit_feature_hint` helper functions."""

load("@bazel_skylib//lib:unittest.bzl", "asserts", "unittest")
load("//features:crubit_feature_hint.bzl", "matches_pattern_for_test")

def _matches_pattern_test_impl(ctx):
    env = unittest.begin(ctx)

    lbl1 = struct(package = "foo/bar", name = "baz")
    asserts.true(env, matches_pattern_for_test(lbl1, "//..."))
    asserts.true(env, matches_pattern_for_test(lbl1, "//foo/bar/..."))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:..."))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:all"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:*"))
    asserts.true(env, matches_pattern_for_test(lbl1, "//foo/bar:baz"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:qux"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/baz/..."))

    lbl2 = struct(package = "foo/bar", name = "bar")
    asserts.true(env, matches_pattern_for_test(lbl2, "//foo/bar"))
    asserts.true(env, matches_pattern_for_test(lbl2, "//foo/..."))

    lbl3 = struct(package = "foo/bar/baz", name = "qux")
    asserts.true(env, matches_pattern_for_test(lbl3, "//foo/bar/..."))
    asserts.false(env, matches_pattern_for_test(lbl3, "//foo/bar:..."))
    asserts.false(env, matches_pattern_for_test(lbl3, "//foo/bar:all"))

    lbl4 = struct(package = "", name = "root_target")
    asserts.true(env, matches_pattern_for_test(lbl4, "//..."))
    asserts.true(env, matches_pattern_for_test(lbl4, "//:root_target"))
    asserts.false(env, matches_pattern_for_test(lbl4, "//:all"))
    asserts.false(env, matches_pattern_for_test(lbl4, "//foo/..."))

    return unittest.end(env)

matches_pattern_test = unittest.make(_matches_pattern_test_impl)

def crubit_feature_hint_test_suite(name):
    unittest.suite(
        name,
        matches_pattern_test,
    )
