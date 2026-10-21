load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "boxes",
    srcs = ["boxes.cc"],
    hdrs = ["boxes.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "//support/rs:box",
        "//support/rs:pin_box",
        "//support/rs:slice_box",
        "//support/rs:str_box",
    ],
)

rust_api_from_cpp(
    name = "boxes_rust_api",
    cpp_target = ":boxes",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":boxes"],
    deps = [
        ":boxes_rust_api",
        "@crate_index//:googletest",
    ],
)
