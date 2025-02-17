load("@rules_rust//rust:defs.bzl", "rust_test")
load("//third_party/protobuf/bazel:cc_proto_library.bzl", "cc_proto_library")
load("//third_party/protobuf/bazel:proto_library.bzl", "proto_library")
load("//third_party/protobuf/rust:defs.bzl", "rust_proto_library")

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
    deps = [":my_cc_proto"],
)

rust_test(
    name = "my_test",
    srcs = ["my_test.rs"],
    cc_deps = [
        ":my_proto_api",
    ],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":my_rust_proto",
        "@crate_index//:googletest",
    ],
)
