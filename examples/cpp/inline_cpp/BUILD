# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//support/extract_cpp_from_rust:rust_library_with_embedded_cpp.bzl", "rust_library_with_embedded_cpp")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library_with_embedded_cpp(
    name = "headers_lib",
    srcs = ["headers.rs"],
    deps_of_cc_library = [
        "@abseil-cpp//absl/strings",
        "//support/rs_std:str_ref",
    ],
)

rust_library_with_embedded_cpp(
    name = "math_lib",
    srcs = ["math.rs"],
)

rust_library_with_embedded_cpp(
    name = "greeting_lib",
    srcs = ["greeting.rs"],
    deps_of_cc_library = [
        "@abseil-cpp//absl/strings",
        "//support/rs_std:str_ref",
    ],
)

rust_library_with_embedded_cpp(
    name = "point_lib",
    srcs = ["point.rs"],
)

rust_library_with_embedded_cpp(
    name = "clamp_lib",
    srcs = ["clamp.rs"],
)

rust_library_with_embedded_cpp(
    name = "math_helper_lib",
    srcs = ["math_helper.rs"],
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],
    deps = [
        ":clamp_lib",
        ":greeting_lib",
        ":math_helper_lib",
        ":math_lib",
        ":point_lib",
    ],
)

rust_test(
    name = "example_test",
    srcs = ["example_test.rs"],
    deps = [
        ":clamp_lib",
        ":greeting_lib",
        ":headers_lib",
        ":math_helper_lib",
        ":math_lib",
        ":point_lib",
        "@crate_index//:googletest",
    ],
)
