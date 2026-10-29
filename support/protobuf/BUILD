"""Protobuf support libraries for Crubit."""

load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@protobuf//rust:defs.bzl", "rust_proto_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
    "crubit_rust_test",
)

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
    deps = [
        "//support:unsafe_relocate_tag",
    ],
)

cc_library(
    name = "cpp_helpers",
    srcs = ["cpp_helpers.cc"],
    hdrs = ["cpp_helpers.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    deps = [
        "@abseil-cpp//absl/log:absl_check",
        "@protobuf//:protobuf_lite",
    ],
)

rust_library(
    name = "proto_cpp",
    srcs = ["cpp.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2024",
    link_deps = [":cpp_helpers"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "//support:ctor",
        "@protobuf//rust:protobuf",
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

rust_proto_library(
    name = "test_rust_proto",
    deps = [":test_proto"],
)

crubit_cc_test(
    name = "rust_test",
    srcs = ["rust_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":rust",
        ":test_cc_proto",
        "//support:movable",
        "@googletest//:gtest_main",
    ],
)

crubit_rust_test(
    name = "cpp_test",
    srcs = ["cpp_test.rs"],
    deps = [
        ":proto_cpp",
        ":test_rust_proto",
        "//support:ctor",
        "@crate_index//:googletest",
        "@protobuf//rust:protobuf",
    ],
)
