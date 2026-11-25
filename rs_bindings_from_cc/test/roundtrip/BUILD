"""End-to-end example of using simple functions."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "roundtrip",
    hdrs = ["roundtrip.h"],
    deps = [
        "//support:annotations",
        "//support/rs_std",
        "//support/rs_std:char",
    ],
)

rust_api_from_cpp(
    name = "roundtrip_rust",
    cpp_target = ":roundtrip",
)

crubit_test_cc_library(
    name = "roundtrip_experimental",
    hdrs = ["roundtrip_experimental.h"],
    aspect_hints = ["//features:experimental"],
    deps = [
        "//support:annotations",
        "//support/rs_std",
    ],
)

rust_api_from_cpp(
    name = "roundtrip_experimental_rust",
    cpp_target = ":roundtrip_experimental",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":roundtrip_experimental_rust",
        ":roundtrip_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
