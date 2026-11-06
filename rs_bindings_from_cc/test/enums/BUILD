load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "enums",
    hdrs = ["enums.h"],
)

rust_api_from_cpp(
    name = "enums_rust_api",
    cpp_target = ":enums",
)

crubit_rust_test(
    name = "enums_test",
    srcs = ["enums_test.rs"],
    deps = [
        ":enums_rust_api",
        "@crate_index//:googletest",
    ],
)
