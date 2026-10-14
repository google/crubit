load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

licenses(["notice"])

cc_library(
    name = "cpp_api",
    srcs = ["cpp_api.cc"],
    hdrs = ["cpp_api.h"],
    aspect_hints = [":cpp_api_rust.hint"],
    deps = [
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

rust_api_from_cpp(
    name = "cpp_api_rust",
    cpp_target = ":cpp_api",
)

rust_test(
    name = "user_of_cpp_api",
    srcs = ["user_of_cpp_api.rs"],
    harness = "libtest",
    deps = [
        ":cpp_api_rust",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "rust_api",
    srcs = ["rust_api.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "rust_api_cc",
    crate = ":rust_api",
)

cc_test(
    name = "user_of_rust_api",
    srcs = ["user_of_rust_api.cc"],
    deps = [
        ":rust_api_cc",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)
