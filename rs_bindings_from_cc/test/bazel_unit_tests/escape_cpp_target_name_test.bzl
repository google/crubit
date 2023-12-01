# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Tests for `escape_cpp_target_name`."""

load("@bazel_skylib//lib:unittest.bzl", "asserts", "unittest")
load(
    "//rs_bindings_from_cc/bazel_support:compile_rust.bzl",
    "escape_cpp_target_name",
)

def _escape_cpp_target_name_test_impl(ctx):
    env = unittest.begin(ctx)
    asserts.equals(env, "foo", escape_cpp_target_name("", "foo"))
    asserts.equals(env, "___________________________", escape_cpp_target_name("", "!./%-@^#$&()*-+,;<=>?[]{|}~"))
    asserts.equals(env, "core_foo_", escape_cpp_target_name("foo~", "core"))
    asserts.equals(env, "bar_", escape_cpp_target_name("foo/bar~", "bar~"))
    asserts.equals(env, "foo_", escape_cpp_target_name("", "foo~"))
    asserts.equals(env, "core_", escape_cpp_target_name("", "core"))
    return unittest.end(env)

escape_cpp_target_name_test = unittest.make(_escape_cpp_target_name_test_impl)

def escape_cpp_target_name_test_suite(name):
    unittest.suite(
        name,
        escape_cpp_target_name_test,
    )
