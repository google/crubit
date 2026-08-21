# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

# `name = "hash"` would make the bindings conflict with a top-level item in the C++ standard library
# See also b/287222576.
rust_library(
    name = "rs_hash",
    testonly = 1,
    srcs = ["hash.rs"],
)

cc_bindings_from_rust(
    name = "rs_hash_cc_api",
    testonly = 1,
    crate = ":rs_hash",
)

golden_test(
    name = "rs_hash_golden_test",
    basename = "rs_hash",
    golden_h = "rs_hash_cc_api.h",
    golden_rs = "rs_hash_cc_api_impl.rs",
    platforms = ["android"],
    rust_library = "rs_hash",
)

crubit_cc_test(
    name = "hash_test",
    srcs = ["hash_test.cc"],
    platforms = ["android"],
    deps = [
        ":rs_hash_cc_api",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/hash",
    ],
)
