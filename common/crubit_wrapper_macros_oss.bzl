# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Wrapper macros needed for Google-internal purposes."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest")
load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("@rules_shell//shell:sh_test.bzl", "sh_test")

crubit_rust_test = rust_test

def crubit_rust_binary(**kwargs):
    # Ignore the `crubit_dep` argument, as the OSS version of
    # `crubit_rust_binary` never inherits a dep on Crubit.
    kwargs.pop("crubit_dep", default = False)
    rust_binary(**kwargs)

crubit_cc_test = cc_test
crubit_sh_test = sh_test
crubit_cc_binary = cc_binary

def _crubit_flavor_transition_impl(_settings, _attr):
    return {}

crubit_flavor_transition = transition(
    implementation = _crubit_flavor_transition_impl,
    inputs = [],
    outputs = [],
)

def _crubit_golden_flavor_transition_impl(settings, attr):
    flags = _crubit_flavor_transition_impl(settings, attr)
    flags["//common/bazel_support:is_golden_test"] = True
    return flags

crubit_golden_flavor_transition = transition(
    implementation = _crubit_golden_flavor_transition_impl,
    inputs = [],
    outputs = ["//common/bazel_support:is_golden_test"],
)
crubit_make_analysis_test = analysistest.make
