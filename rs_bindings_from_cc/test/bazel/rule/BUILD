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

licenses(["notice"])

cc_library(
    name = "lib",
    srcs = ["lib.cc"],
    hdrs = ["lib.h"],
    aspect_hints = ["//features:supported"],
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
    hdrs = ["cpp_add.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "my_lib_rust",
    cpp_deps = [":my_lib"],
    cpp_hdrs = ["my_lib_rust.h"],
    cpp_target = ":my_lib",
)

rust_test(
    name = "my_lib_rust_test",
    srcs = ["my_lib_rust_test.rs"],
    deps = [":my_lib_rust"],
)

rust_api_from_cpp(
    name = "my_lib_rust_auto",
    srcs = ["my_lib.rs"],
    cpp_target = ":my_lib",
)

rust_test(
    name = "my_lib_rust_auto_test",
    srcs = ["my_lib_rust_auto_test.rs"],
    deps = [":my_lib_rust_auto"],
)
