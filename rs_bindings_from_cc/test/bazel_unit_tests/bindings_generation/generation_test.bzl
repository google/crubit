# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A test that the dependencies needed for our generated bindings files are built in target cfg."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_make_analysis_test")
load("//rs_bindings_from_cc/bazel_support:providers.bzl", "RustBindingsFromCcInfo")
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "attach_aspect",
)
load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")

def _bindings_generated_when_public_headers_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    asserts.true(env, RustBindingsFromCcInfo in tut, "expected target to have RustBindingFromCcInfo")
    bindings_info = tut[RustBindingsFromCcInfo]
    asserts.true(env, bindings_info.dep_variant_info, "expected target to have DepVariantInfo")

    return analysistest.end(env)

bindings_generated_when_public_headers_test = crubit_make_analysis_test(
    _bindings_generated_when_public_headers_impl,
)

def _bindings_generated_when_public_headers():
    native.cc_library(name = "has_pub_headers", hdrs = ["lib.h"], aspect_hints = ["//features:supported"])
    attach_aspect(name = "has_pub_headers_with_aspect", dep = ":has_pub_headers")
    bindings_generated_when_public_headers_test(
        name = "bindings_generated_when_public_headers_test",
        target_under_test = ":has_pub_headers_with_aspect",
    )

def _bindings_not_generated_when_no_public_headers_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    asserts.true(env, RustBindingsFromCcInfo in tut, "expected target to have RustBindingFromCcInfo")
    bindings_info = tut[RustBindingsFromCcInfo]
    asserts.false(env, bindings_info.dep_variant_info, "expected target not to have DepVariantInfo")

    return analysistest.end(env)

bindings_not_generated_when_no_public_headers_test = crubit_make_analysis_test(
    _bindings_not_generated_when_no_public_headers_impl,
)

def _bindings_not_generated_when_no_public_headers():
    native.cc_library(name = "no_pub_headers", srcs = ["no_pub_headers.cc"], alwayslink = 1, aspect_hints = ["//features:supported"])
    attach_aspect(name = "no_pub_headers_with_aspect", dep = ":no_pub_headers")
    bindings_not_generated_when_no_public_headers_test(
        name = "bindings_not_generated_when_no_public_headers_test",
        target_under_test = ":no_pub_headers_with_aspect",
    )

def _bindings_not_generated_for_protobufs_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    asserts.true(env, RustBindingsFromCcInfo in tut, "expected target to have RustBindingFromCcInfo")
    bindings_info = tut[RustBindingsFromCcInfo]
    asserts.false(env, bindings_info.dep_variant_info, "expected target not to have DepVariantInfo")

    return analysistest.end(env)

bindings_not_generated_for_protobufs_test = crubit_make_analysis_test(
    _bindings_not_generated_for_protobufs_impl,
)

def _bindings_not_generated_for_protobufs():
    proto_library(name = "my_proto", srcs = ["my.proto"])
    cc_proto_library(name = "my_cc_proto", deps = [":my_proto"])
    attach_aspect(name = "my_cc_proto_with_aspect", dep = ":my_cc_proto")
    bindings_not_generated_for_protobufs_test(
        name = "bindings_not_generated_for_protobufs_test",
        target_under_test = ":my_cc_proto_with_aspect",
    )

def generation_test(name):
    """Sets up the test suite.

    Args:
      name: name of the test suite"""
    _bindings_generated_when_public_headers()
    _bindings_not_generated_when_no_public_headers()
    _bindings_not_generated_for_protobufs()

    native.test_suite(
        name = name,
        tests = [
            ":bindings_generated_when_public_headers_test",
            ":bindings_not_generated_when_no_public_headers_test",
            ":bindings_not_generated_for_protobufs_test",
        ],
    )
