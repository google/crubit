load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:public"],
)

bzl_library(
    name = "cpp_api_from_rust_bzl",
    srcs = ["cpp_api_from_rust.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule_bzl",
    ],
)

bzl_library(
    name = "rust_api_from_cpp_bzl",
    srcs = ["rust_api_from_cpp.bzl"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp",
    ],
)
