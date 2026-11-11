load(
    "@rules_cc//cc:cc_library.bzl",
    "cc_library",
)
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_struct",
    hdrs = ["cc_struct.h"],
    aspect_hints = [":cc_struct_rust.hint"],
)

rust_api_from_cpp(
    name = "cc_struct_rust",
    testonly = 1,
    cpp_target = ":cc_struct",
)

rust_library(
    name = "dynamic_field",
    testonly = 1,
    srcs = ["lib.rs"],
    edition = "2024",
    deps = [":cc_struct_rust"],
)

cpp_api_from_rust(
    name = "dynamic_field_cc_api",
    testonly = 1,
    crate = ":dynamic_field",
)

crubit_cc_test(
    name = "test",
    srcs = ["test.cc"],
    tags = ["no_test_android_x86"],
    deps = [
        ":dynamic_field_cc_api",
        "@googletest//:gtest_main",
    ],
)
