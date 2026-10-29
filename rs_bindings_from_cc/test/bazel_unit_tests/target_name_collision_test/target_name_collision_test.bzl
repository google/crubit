# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Targets whose names escape to the same crate name get distinct generated files.

`foo-bar`, `foo_bar` and `foo/bar` all have the crate name `foo_bar`. If the generated files were
named after the crate, or after the raw target name (the files of `foo/bar` are compiled under
their basename, `bar_...`), their bindings would declare the same files, and analyzing them
together (as this test suite does) would fail with conflicting actions.
"""

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

def _generated_files_test_impl(ctx):
    env = analysistest.begin(ctx)
    info = analysistest.target_under_test(env)[GeneratedBindingsInfo]
    stem = ctx.attr.expected_stem
    asserts.equals(env, stem + "_rust_api_impl.cc", info.cc_file.basename)
    asserts.equals(env, stem + "_rust_api.rs", info.rust_file.basename)
    asserts.equals(env, stem + "_namespaces.json", info.namespaces_file.basename)
    return analysistest.end(env)

_generated_files_test = crubit_make_analysis_test(
    _generated_files_test_impl,
    attrs = {"expected_stem": attr.string()},
)

def target_name_collision_test(name):
    """Sets up the target_name_collision test suite.

    Args:
      name: name of the test suite
    """
    targets = {
        # target name: expected file stem
        "foo-bar": "foo_d_bar",
        "foo_bar": "foo_bar",
        "foo/bar": "foo_y_bar",
        "bar": "bar",
    }
    for target, stem in targets.items():
        cc_library(
            name = target,
            hdrs = ["lib.h"],
            aspect_hints = ["//features:supported"],
        )
        attach_aspect(name = target + "_bindings", dep = ":" + target)
        _generated_files_test(
            name = target + "_test",
            target_under_test = ":" + target + "_bindings",
            expected_stem = stem,
        )

    native.test_suite(
        name = name,
        tests = [":" + target + "_test" for target in targets],
    )
