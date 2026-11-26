load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "vec",
    srcs = ["vec.cc"],
    hdrs = ["vec.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support/rs:rs_alloc",
        "//support/rs:vec",
    ],
)

rust_api_from_cpp(
    name = "vec_rust_api",
    cpp_target = ":vec",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":vec_rust_api",
        "@crate_index//:googletest",
    ],
)
