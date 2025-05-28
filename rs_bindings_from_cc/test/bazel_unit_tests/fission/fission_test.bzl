# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)

def _remove_trailing_hash(s):
    "Strips the trailing output hash from names like rust_library_fission234234"
    if s.endswith(".a"):
        return s.removesuffix(".a").rstrip("-0123456789") + ".a"
    return s.rstrip("-0123456789")

def _fission_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    expected = sorted([
        "has_no_public_headers.pic.dwo",
        "enabled_crubit_has_public_headers.pic.dwo",
        "enabled_crubit_has_public_headers_rust_api_impl.pic.dwo",
        "rust_library_fission",
        # plus, for good measure, also one of the support libraries needed by the bindings.
        # The C++ support library is header-only, but the Rust support libraries do have object
        # files. For example:
        "oops_fission",  # Object Oriented Programming Support for Rust.
    ])

    should_not_exist = [
        "has_no_public_headers_rust_api_impl.pic.dwo",
        "disabled_crubit_rust_api_impl.pic.dwo",
    ]
    actual = sorted([
        _remove_trailing_hash(f.basename)
        for f in target_under_test[CcInfo].debug_context().pic_files.to_list()
        # The full list is too large and fragile to test -- this would be a change-detector -- but
        # we can test representative examples of everything we expect in it.
        if _remove_trailing_hash(f.basename) in expected
    ])

    actual_not_exist = [
        _remove_trailing_hash(f.basename)
        for f in target_under_test[CcInfo].debug_context().pic_files.to_list()
        if _remove_trailing_hash(f.basename) in should_not_exist
    ]

    asserts.equals(
        env,
        expected = [],
        actual = actual_not_exist,
    )
    asserts.equals(
        env,
        expected = expected,
        actual = actual,
    )

    linker_inputs_should_not_exist = [
        "has_no_public_headers_rust_api_impl.pic.o",
        "disabled_crubit_rust_api_impl.pic.o",
        "libhas_no_public_headers.a",
        "libdisabled_crubit.a",
    ]
    all_linker_inputs = []
    for linker_input in target_under_test[CcInfo].linking_context.linker_inputs.to_list():
        for lib in linker_input.libraries:
            if lib.static_library:
                all_linker_inputs.append(lib.static_library)
            if lib.objects:
                all_linker_inputs.extend(lib.objects)
            if lib.pic_objects:
                all_linker_inputs.extend(lib.pic_objects)

    actual_linker_inputs = sorted([
        _remove_trailing_hash(f.basename)
        for f in all_linker_inputs
        if _remove_trailing_hash(f.basename) in linker_inputs_should_not_exist
    ])
    asserts.equals(
        env,
        expected = [],
        actual = actual_linker_inputs,
    )

    return analysistest.end(env)

fission_test = crubit_make_analysis_test(
    _fission_test_impl,
    config_settings = {
        "//command_line_option:fission": ["yes"],
    },
)

def _test_fission():
    # Enables Crubit, but has no public headers.
    native.cc_library(name = "has_no_public_headers", tags = ["manual"], srcs = ["has_no_public_headers.cc"], aspect_hints = ["//features:supported"])

    # Has public headers, but does not have Crubit enabled.
    native.cc_library(name = "disabled_crubit", tags = ["manual"], hdrs = ["disabled_crubit.h"])

    # Has public headers, and enables Crubit.
    native.cc_library(name = "enabled_crubit_has_public_headers", tags = ["manual"], srcs = ["enabled_crubit_has_public_headers.cc"], hdrs = ["enabled_crubit_has_public_headers.h"], deps = [":has_no_public_headers", ":disabled_crubit"], aspect_hints = ["//features:supported"])
    rust_library(name = "rust_library", tags = ["manual"], cc_deps = [":enabled_crubit_has_public_headers"], srcs = ["rust_library.rs"])

    fission_test(
        name = "fission_test",
        target_under_test = ":rust_library",
        # Fission isn't supported on macOS, see b/312277119.
        tags = ["not_run:mac"],
    )

# Tests that Crubit correctly forwards Fission debug info (https://gcc.gnu.org/wiki/DebugFission)
# from a C++ library.
def fission_test_suite(name):
    _test_fission()
    native.test_suite(
        name = name,
        tests = [
            ":fission_test",
        ],
    )
