# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//support/extract_cpp_from_rust:rust_library_with_embedded_cpp.bzl", "rust_library_with_embedded_cpp")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library_with_embedded_cpp(
    name = "example_lib",
    srcs = ["example.rs"],
    deps_of_cc_library = [
        "@abseil-cpp//absl/strings",
        "//support/rs_std:str_ref",
    ],
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],
    deps = [":example_lib"],
)

rust_test(
    name = "example_test",
    srcs = ["example_test.rs"],
    deps = [
        ":example_lib",
        "@crate_index//:googletest",
    ],
)
