# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Wrapper macros needed for Google-internal purposes."""

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("@bazel_skylib//lib:unittest.bzl", "analysistest")

crubit_rust_test = rust_test

def crubit_rust_binary(**kwargs):
    # Ignore the `crubit_dep` argument, as the OSS version of
    # `crubit_rust_binary` never inherits a dep on Crubit.
    kwargs.pop("crubit_dep", default = False)
    rust_binary(**kwargs)

crubit_cc_test = native.cc_test
crubit_sh_test = native.sh_test
crubit_cc_binary = native.cc_binary
crubit_flavor_transition = transition(
    implementation = lambda _settings, _attr: {},
    inputs = [],
    outputs = [],
)
crubit_make_analysis_test = analysistest.make
