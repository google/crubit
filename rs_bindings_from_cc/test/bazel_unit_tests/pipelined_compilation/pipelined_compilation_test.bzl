# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""This module contains unit tests for pipelined compilation of bindings."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_make_analysis_test",
)
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _contains_input_of_type(name, input_type, inputs):
    return len([
        i
        for i in inputs
        if i.extension == input_type and i.basename.startswith("lib" + name)
    ]) > 0

def _action_inputs_with_pipelining_analysis_test_impl(ctx):
    env = analysistest.begin(ctx)
    target_under_test = analysistest.target_under_test(env)

    metadata_action = [
        a
        for a in target_under_test[ActionsInfo].actions
        if a.mnemonic == "RustcMetadata"
    ][0]

    # Check that we generate metadata for our binding
    asserts.true(env, metadata_action != None)

    metadata_action_inputs = metadata_action.inputs.to_list()

    # Check that our metadata action accepts metadata inputs from the dependencies
    asserts.true(env, _contains_input_of_type("middle", "rmeta", metadata_action_inputs))
    asserts.true(env, _contains_input_of_type("top", "rmeta", metadata_action_inputs))
    asserts.false(env, _contains_input_of_type("middle", "rlib", metadata_action_inputs))
    asserts.false(env, _contains_input_of_type("top", "rlib", metadata_action_inputs))

    rlib_action = [a for a in target_under_test[ActionsInfo].actions if a.mnemonic == "Rustc"][0]
    rlib_action_inputs = rlib_action.inputs.to_list()

    # Check that the rlib action accepts metadata inputs from the dependencies
    asserts.true(env, _contains_input_of_type("middle", "rmeta", rlib_action_inputs))
    asserts.true(env, _contains_input_of_type("top", "rmeta", rlib_action_inputs))
    asserts.false(env, _contains_input_of_type("middle", "rlib", rlib_action_inputs))
    asserts.false(env, _contains_input_of_type("top", "rlib", rlib_action_inputs))

    return analysistest.end(env)

action_inputs_with_pipelining_analysis_test = crubit_make_analysis_test(
    _action_inputs_with_pipelining_analysis_test_impl,
)

def _targets_for_pipelined_compilation():
    native.cc_library(
        name = "top",
        hdrs = [
            "top.h",
        ],
    )
    native.cc_library(
        name = "middle",
        hdrs = [
            "middle.h",
        ],
        deps = [":top"],
    )
    native.cc_library(
        name = "bottom",
        hdrs = ["bottom.h"],
        deps = [":middle"],
    )

    attach_aspect(name = "pipelined_compilation_of_bindings", dep = ":bottom")

    action_inputs_with_pipelining_analysis_test(
        name = "pipelined_compilation_of_bindings_test",
        target_under_test = ":pipelined_compilation_of_bindings",
    )

def pipelined_compilation_test(name):
    """Sets up rust_bindings_from_cc_aspect test suite.

    Args:
      name: name of the test suite"""
    _targets_for_pipelined_compilation()

    native.test_suite(
        name = name,
        tests = [
            ":pipelined_compilation_of_bindings_test",
        ],
    )
