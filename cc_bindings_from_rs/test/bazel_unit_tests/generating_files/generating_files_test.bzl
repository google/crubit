# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for the main generated files of cc_bindings_from_rs."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

def _header_generation_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    asserts.true(env, CcInfo in target_under_test)
    cc_info = target_under_test[CcInfo]
    asserts.true(env, len(cc_info.compilation_context.direct_headers) == 1)
    generated_header = cc_info.compilation_context.direct_headers[0]
    asserts.true(env, generated_header.path.endswith("rusty_lib_cc_api.h"))

    return analysistest.end(env)

header_generation_test = analysistest.make(_header_generation_test_impl)

def _tests():
    rust_library(
        name = "rusty_lib",
        srcs = ["lib.rs"],
    )

    cc_bindings_from_rust(
        name = "rusty_lib_bindings",
        crate = ":rusty_lib",
    )

    header_generation_test(
        name = "header_generation_test",
        target_under_test = ":rusty_lib_bindings",
    )

def generating_files_test(name):
    """Sets up generating_files_test test suite.

    Args:
      name: name of the test suite"""
    _tests()

    native.test_suite(
        name = name,
        tests = [
            ":header_generation_test",
        ],
    )
