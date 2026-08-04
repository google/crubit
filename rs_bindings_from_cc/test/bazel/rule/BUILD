load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_test")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

cc_library(
    name = "lib",
    srcs = ["lib.cc"],
    hdrs = ["lib.h"],
    aspect_hints = [":lib_rust.hint"],
)

rust_api_from_cpp(
    name = "lib_rust",
    cpp_target = ":lib",
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)

crubit_rust_test(
    name = "crubit_test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)

cc_library(
    name = "my_lib",
    hdrs = ["math_dependency.h"],
    aspect_hints = [
        ":my_lib_rust.hint",
    ],
)

rust_api_from_cpp(
    name = "my_lib_rust",
    srcs = ["additional_rust_srcs.rs"],
    cpp_srcs = ["additional_cpp_srcs.cc"],
    cpp_target = ":my_lib",
)

crubit_rust_test(
    name = "my_lib_rust_test",
    srcs = ["additional_rust_srcs_test.rs"],
    deps = [
        ":my_lib_rust",
        "@crate_index//:googletest",
    ],
)
