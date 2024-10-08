# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)

def _remove_trailing_hash(s):
    "Strips the trailing output hash from names like rust_library_fission234234"

    return s.rstrip("-0123456789")

def _fission_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    expected = [
        "cc_library.pic.dwo",  # the C++ library
        "rust_library_1_fission",
        "rust_library_1_cc_api_impl_fission",  # the generated bindings for C++
        "rust_library_2_fission",
        "rust_library_2_cc_api_impl_fission",  # the generated bindings for C++
    ]
    actual = [
        _remove_trailing_hash(f.basename)
        for f in target_under_test[CcInfo].debug_context().pic_files.to_list()
    ]
    for fname in expected:
        asserts.true(env, fname in actual, "expected pic file " + fname + " is missing")

    return analysistest.end(env)

fission_test = crubit_make_analysis_test(
    _fission_test_impl,
    config_settings = {
        "//command_line_option:fission": ["yes"],
    },
)

def _test_fission():
    rust_library(name = "rust_library_1", tags = ["manual"], srcs = ["rust_library_1.rs"])
    rust_library(name = "rust_library_2", tags = ["manual"], srcs = ["rust_library_2.rs"], deps = [":rust_library_1"])
    cc_bindings_from_rust(
        name = "rust_library_2_cc_api",
        tags = ["manual"],
        crate = ":rust_library_2",
    )
    native.cc_library(name = "cc_library", tags = ["manual"], deps = [":rust_library_2_cc_api"], srcs = ["cc_library.cc"])

    fission_test(
        name = "fission_test",
        target_under_test = ":cc_library",
    )

def fission_test_suite(name):
    _test_fission()
    native.test_suite(
        name = name,
        tests = [
            ":fission_test",
        ],
    )
