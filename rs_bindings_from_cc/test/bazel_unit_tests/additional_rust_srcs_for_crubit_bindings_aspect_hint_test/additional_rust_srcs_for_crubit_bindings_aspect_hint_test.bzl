# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for extra_rs_srcs aspect hint."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
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
    cc_library(
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
    cc_library(
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

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate():
    additional_rust_srcs_for_crubit_bindings(
        name = "my_cc_lib_additional_rust_srcs",
        srcs = [
            "my_cc_lib_rust_api.rs",
        ],
        tags = ["manual"],
        deps = [":a_rust_lib_dep"],
        link_deps = [":a_cc_link_dep"],
        cc_deps = [":aspect_for_cc_dep_lib_with_crubit"],
    )
    cc_library(
        name = "my_cc_lib_target",
        hdrs = ["my_cc_lib.h"],
        aspect_hints = [
            ":my_cc_lib_additional_rust_srcs",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_my_cc_lib_target",
        dep = ":my_cc_lib_target",
    )

    cc_library(
        name = "a_cc_link_dep",
        hdrs = ["empty.h"],
    )

    additional_rust_srcs_for_crubit_bindings(
        name = "cc_dep_lib_additional_rust_srcs",
        srcs = [
            "cc_dep_lib_rust_api.rs",
        ],
        tags = ["manual"],
    )
    cc_library(
        name = "cc_dep_lib_with_crubit",
        hdrs = ["cc_dep_lib.h"],
        aspect_hints = [
            ":cc_dep_lib_additional_rust_srcs",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_dep_lib_with_crubit",
        dep = ":cc_dep_lib_with_crubit",
    )

    rust_library(
        name = "a_rust_lib_dep",
        srcs = ["a_rust_lib.rs"],
    )

    additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_test(
        name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_test",
        target_under_test = ":aspect_for_my_cc_lib_target",
    )

def _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    analysis_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "Rustc"][0]

    expected_flag_prefix_deps = "--extern=a_rust_lib_dep="
    expected_flag_prefix_cc_deps = "--extern=cc_dep_lib_with_crubit="
    not_expected_flag_prefix_link_deps = "--extern=a_cc_link_dep="
    asserts.true(
        env,
        True in [flag.startswith(expected_flag_prefix_deps) for flag in analysis_action.argv],
        "Flag starting with '%s' failed to be passed to rs_bindings_from_cc_driver. Actual flags: %s" % (expected_flag_prefix_deps, analysis_action.argv),
    )
    asserts.true(
        env,
        True in [flag.startswith(expected_flag_prefix_cc_deps) for flag in analysis_action.argv],
        "Flag starting with '%s' failed to be passed to rs_bindings_from_cc_driver. Actual flags: %s" % (expected_flag_prefix_cc_deps, analysis_action.argv),
    )
    asserts.false(
        env,
        True in [flag.startswith(not_expected_flag_prefix_link_deps) for flag in analysis_action.argv],
        "Flag starting with '%s' should not be passed to rs_bindings_from_cc_driver. Actual flags: %s" % (not_expected_flag_prefix_link_deps, analysis_action.argv),
    )
    return analysistest.end(env)

additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_test = crubit_make_analysis_test(_test_additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_impl)

def _test_rust_api_from_cpp_srcs_set_crubit_target():
    cc_library(
        name = "cc_lib_for_crubit_target",
        hdrs = ["empty.h"],
        aspect_hints = [":cc_lib_for_crubit_target_rust.hint"],
        tags = ["manual"],
    )
    rust_api_from_cpp(
        name = "cc_lib_for_crubit_target_rust",
        cpp_target = ":cc_lib_for_crubit_target",
        srcs = ["stub.rs"],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_lib_for_crubit_target",
        dep = ":cc_lib_for_crubit_target",
    )
    crubit_target_env_test(
        name = "rust_api_from_cpp_srcs_set_crubit_target_test",
        target_under_test = ":aspect_for_cc_lib_for_crubit_target",
        # Without `embedded_cpp`, the aspect falls back to the C++ target's label.
        expected_crubit_target = str(native.package_relative_label(":cc_lib_for_crubit_target")),
    )

    cc_library(
        name = "cc_lib_without_srcs_for_crubit_target",
        hdrs = ["empty.h"],
        aspect_hints = [":cc_lib_without_srcs_for_crubit_target_rust.hint"],
        tags = ["manual"],
    )
    rust_api_from_cpp(
        name = "cc_lib_without_srcs_for_crubit_target_rust",
        cpp_target = ":cc_lib_without_srcs_for_crubit_target",
        # Setting `root_namespaces` (like `srcs` or `deps`) makes `.hint` an
        # `additional_rust_srcs_for_crubit_bindings` with empty `srcs`. Without it,
        # `rust_api_from_cpp` makes `.hint` an alias of its single `crubit_features` hint, and the
        # aspect never sees an `AdditionalRustSrcsProviderInfo`.
        root_namespaces = ["unused"],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_lib_without_srcs_for_crubit_target",
        dep = ":cc_lib_without_srcs_for_crubit_target",
    )
    crubit_target_env_test(
        name = "rust_api_from_cpp_without_srcs_no_crubit_target_test",
        target_under_test = ":aspect_for_cc_lib_without_srcs_for_crubit_target",
        expected_crubit_target = "",
    )

def _crubit_target_env_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)
    rustc_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "Rustc"][0]
    asserts.equals(
        env,
        ctx.attr.expected_crubit_target,
        rustc_action.env.get("CRUBIT_TARGET", ""),
        "Unexpected CRUBIT_TARGET in the bindings Rustc action env",
    )
    return analysistest.end(env)

crubit_target_env_test = crubit_make_analysis_test(
    _crubit_target_env_test_impl,
    attrs = {"expected_crubit_target": attr.string()},
)

def _test_conflicting_crubit_targets_fail():
    additional_rust_srcs_for_crubit_bindings(
        name = "crubit_target_a_hint",
        srcs = ["stub.rs"],
        crubit_target = "//pkg:a",
        tags = ["manual"],
    )
    additional_rust_srcs_for_crubit_bindings(
        name = "crubit_target_b_hint",
        srcs = ["inner.rs"],
        crubit_target = "//pkg:b",
        tags = ["manual"],
    )
    cc_library(
        name = "cc_lib_with_conflicting_crubit_targets",
        hdrs = ["empty.h"],
        aspect_hints = [
            ":crubit_target_a_hint",
            ":crubit_target_b_hint",
        ],
        tags = ["manual"],
    )
    attach_aspect(
        name = "aspect_for_cc_lib_with_conflicting_crubit_targets",
        dep = ":cc_lib_with_conflicting_crubit_targets",
        tags = ["manual"],
    )
    conflicting_crubit_targets_fail_test(
        name = "conflicting_crubit_targets_fail_test",
        target_under_test = ":aspect_for_cc_lib_with_conflicting_crubit_targets",
    )

def _conflicting_crubit_targets_fail_impl(ctx):
    env = analysistest.begin(ctx)
    asserts.expect_failure(env, "aspect hints set different `crubit_target` values")
    return analysistest.end(env)

conflicting_crubit_targets_fail_test = crubit_make_analysis_test(
    _conflicting_crubit_targets_fail_impl,
    expect_failure = True,
)

def additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(name):
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli()
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers()
    _test_additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate()
    _test_rust_api_from_cpp_srcs_set_crubit_target()
    _test_conflicting_crubit_targets_fail()
    native.test_suite(
        name = name,
        tests = [
            ":additional_rust_srcs_for_crubit_bindings_aspect_hint_propagate_to_cli_test",
            ":test_additional_rust_srcs_for_crubit_bindings_aspect_hint_generates_bindings_when_no_public_headers_test",
            ":additional_rust_srcs_for_crubit_bindings_aspect_hint_deps_and_cc_deps_propagate_test",
            ":rust_api_from_cpp_srcs_set_crubit_target_test",
            ":rust_api_from_cpp_without_srcs_no_crubit_target_test",
            ":conflicting_crubit_targets_fail_test",
        ],
    )
