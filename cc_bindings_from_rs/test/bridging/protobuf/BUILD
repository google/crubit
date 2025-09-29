"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging Protobuf types."""

load("@rules_proto//proto:defs.bzl", "proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@protobuf//rust:defs.bzl", "rust_proto_library")
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

proto_library(
    name = "foo_proto",
    testonly = True,
    srcs = ["foo.proto"],
)

rust_proto_library(
    name = "foo_rust_proto",
    testonly = True,
    deps = [":foo_proto"],
)

cc_proto_library(
    name = "foo_cc_proto",
    testonly = True,
    deps = [":foo_proto"],
)

rust_library(
    name = "rust_lib",
    testonly = True,
    srcs = ["rust_lib.rs"],
    aspect_hints = ["//features:experimental"],
    deps = [":foo_rust_proto"],
)

cc_bindings_from_rust(
    name = "rust_lib_cc_api",
    testonly = True,
    crate = ":rust_lib",
)

golden_test(
    name = "rust_lib_golden_test",
    basename = "rust_lib",
    golden_h = "rust_lib_cc_api.h",
    golden_rs = "rust_lib_cc_api_impl.rs",
    rust_library = "rust_lib",
)

crubit_cc_test(
    name = "user_of_rust_lib",
    srcs = ["user_of_rust_lib.cc"],
    deps = [
        ":foo_cc_proto",
        ":rust_lib_cc_api",
        "@googletest//:gtest_main",
    ],
)
