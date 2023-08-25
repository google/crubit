# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Wrapper macros needed for Google-internal purposes."""

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")

crubit_rust_test = rust_test
crubit_rust_binary = rust_binary
crubit_cc_test = native.cc_test
crubit_cc_binary = native.cc_binary
crubit_flavor_transition = transition(
    implementation = lambda _settings, _attr: {},
    inputs = [],
    outputs = [],
)
crubit_on_demand_rust_test = rust_test
crubit_on_demand_cc_test = native.cc_test
