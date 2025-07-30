# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A test that the dependencies needed for our generated bindings files are built in target cfg."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
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
    cc_library(name = "has_pub_headers", hdrs = ["lib.h"], aspect_hints = ["//features:supported"])
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
    cc_library(name = "no_pub_headers", srcs = ["no_pub_headers.cc"], alwayslink = 1, aspect_hints = ["//features:supported"])
    attach_aspect(name = "no_pub_headers_with_aspect", dep = ":no_pub_headers")
    bindings_not_generated_when_no_public_headers_test(
        name = "bindings_not_generated_when_no_public_headers_test",
        target_under_test = ":no_pub_headers_with_aspect",
    )

def _protoc_bindings_used_for_protobufs_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    asserts.true(env, RustBindingsFromCcInfo in tut, "expected target to have RustBindingFromCcInfo")
    bindings_info = tut[RustBindingsFromCcInfo]

    references_protobuf_cpp = any([
        "@protobuf//rust:protobuf_cpp" == str(key.label)
        for key in bindings_info.dep_variant_info.crate_info.aliases.keys()
    ])

    asserts.true(env, references_protobuf_cpp, "Expected target to reference protobuf_cpp alias")

    return analysistest.end(env)

protoc_bindings_used_for_protobufs_test = crubit_make_analysis_test(
    _protoc_bindings_used_for_protobufs_impl,
)

def _protoc_bindings_used_for_protobufs():
    proto_library(name = "my_proto", srcs = ["my.proto"])
    cc_proto_library(name = "my_cc_proto", deps = [":my_proto"])
    attach_aspect(name = "my_cc_proto_with_aspect", dep = ":my_cc_proto")
    protoc_bindings_used_for_protobufs_test(
        name = "protoc_bindings_used_for_protobufs_test",
        target_under_test = ":my_cc_proto_with_aspect",
    )

def _protoc_bindings_forwaded_for_alias_library_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    asserts.true(env, RustBindingsFromCcInfo in tut, "expected target to have RustBindingFromCcInfo")
    bindings_info = tut[RustBindingsFromCcInfo]

    asserts.true(env, len(bindings_info.pass_through_dep_variant_infos.to_list()) == 2, "Expected target to pass through 2 DepVariantInfos")

    return analysistest.end(env)

protoc_bindings_forwaded_for_alias_library_test = crubit_make_analysis_test(
    _protoc_bindings_forwaded_for_alias_library_impl,
)

def _protoc_bindings_forwaded_for_alias_library():
    # already setup by _protoc_bindings_used_for_protobufs
    # redeclaring this breaks something
    # proto_library(name = "my_alias_proto", srcs = ["my.proto"])

    proto_library(name = "other_proto", srcs = ["other.proto"])
    proto_library(name = "merged_proto", deps = [":my_proto", ":other_proto"])
    cc_proto_library(name = "merged_cc_proto", deps = [":merged_proto"])
    attach_aspect(name = "merged_cc_proto_with_aspect", dep = ":merged_cc_proto")
    protoc_bindings_forwaded_for_alias_library_test(
        name = "protoc_bindings_forwaded_for_alias_library_test",
        target_under_test = ":merged_cc_proto_with_aspect",
    )

def generation_test(name):
    """Sets up the test suite.

    Args:
      name: name of the test suite"""
    _bindings_generated_when_public_headers()
    _bindings_not_generated_when_no_public_headers()
    _protoc_bindings_used_for_protobufs()
    _protoc_bindings_forwaded_for_alias_library()

    native.test_suite(
        name = name,
        tests = [
            ":bindings_generated_when_public_headers_test",
            ":bindings_not_generated_when_no_public_headers_test",
            ":protoc_bindings_used_for_protobufs_test",
            ":protoc_bindings_forwaded_for_alias_library_test",
        ],
    )
