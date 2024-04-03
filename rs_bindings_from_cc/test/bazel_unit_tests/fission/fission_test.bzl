# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)

def _fission_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    expected = sorted([
        "cc_library_1.pic.dwo",
        "cc_library_1_rust_api_impl.pic.dwo",
        "cc_library_2.pic.dwo",
        "cc_library_2_rust_api_impl.pic.dwo",
        "rust_library_fission",
        # plus, for good measure, also one of the support libraries needed by the bindings.
        # The C++ support library is header-only, but the Rust support libraries do have object
        # files. For example:
        "oops_fission",  # Object Oriented Programming Support for Rust.
    ])
    actual = sorted([
        f.basename
        for f in target_under_test[CcInfo].debug_context().pic_files.to_list()
        # The full list is too large and fragile to test -- this would be a change-detector -- but
        # we can test representative examples of everything we expect in it.
        if f.basename in expected
    ])
    asserts.equals(
        env,
        expected = expected,
        actual = actual,
    )

    return analysistest.end(env)

fission_test = crubit_make_analysis_test(
    _fission_test_impl,
    config_settings = {
        "//command_line_option:fission": ["yes"],
    },
)

def _test_fission():
    native.cc_library(name = "cc_library_1", tags = ["manual"], srcs = ["cc_library_1.cc"])
    native.cc_library(name = "cc_library_2", tags = ["manual"], srcs = ["cc_library_2.cc"], deps = [":cc_library_1"])
    rust_library(name = "rust_library", tags = ["manual"], cc_deps = [":cc_library_2"], srcs = ["rust_library.rs"])

    fission_test(
        name = "fission_test",
        target_under_test = ":rust_library",
    )

def fission_test_suite(name):
    _test_fission()
    native.test_suite(
        name = name,
        tests = [
            ":fission_test",
        ],
    )
