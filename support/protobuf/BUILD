"""Protobuf support libraries for Crubit."""

load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

exports_files(
    srcs = glob(["*"]),
    visibility = [
        ":__subpackages__",
    ],
)

cc_library(
    name = "rust",
    hdrs = ["rust.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)

proto_library(
    name = "test_proto",
    srcs = ["test.proto"],
)

cc_proto_library(
    name = "test_cc_proto",
    deps = [":test_proto"],
)

crubit_cc_test(
    name = "rust_test",
    srcs = ["rust_test.cc"],
    deps = [
        ":rust",
        ":test_cc_proto",
        "//testing/base/public:gunit_main",
    ],
)
