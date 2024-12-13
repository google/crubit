"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging Protobuf types."""

load("@rules_proto//proto:defs.bzl", "proto_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load("//third_party/protobuf/bazel:proto_library.bzl", "proto_library")
load("//third_party/protobuf/rust:defs.bzl", "rust_proto_library")

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
    name = "foo_lib",
    testonly = True,
    srcs = ["foo_lib.rs"],
    aspect_hints = ["//features:experimental"],
    deps = [":foo_rust_proto"],
)

cc_bindings_from_rust(
    name = "foo_lib_cc_api",
    testonly = True,
    crate = ":foo_lib",
)

crubit_cc_test(
    name = "foo_lib_test",
    srcs = ["foo_lib_test.cc"],
    deps = [
        ":foo_cc_proto",
        ":foo_lib_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
