load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@protobuf//rust:defs.bzl", "rust_proto_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

proto_library(
    name = "my_proto",
    srcs = ["my.proto"],
)

cc_proto_library(
    name = "my_cc_proto",
    deps = [":my_proto"],
)

rust_proto_library(
    name = "my_rust_proto",
    deps = [":my_proto"],
)

cc_library(
    name = "my_proto_api",
    hdrs = ["my_proto_api.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":my_cc_proto",
    ],
)

crubit_rust_test(
    name = "my_test",
    srcs = ["my_test.rs"],
    cc_deps = [
        ":my_proto_api",
    ],
    deps = [
        ":my_rust_proto",
        "//support:forward_declare",
        "@crates//:googletest",
    ],
)
