load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "sfinae",
    hdrs = ["sfinae.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "sfinae_rust",
    cpp_target = ":sfinae",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":sfinae_rust",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "atomic_regression_cc",
    hdrs = ["atomic_regression.h"],
    aspect_hints = ["//features:template_instantiation"],
)

rust_api_from_cpp(
    name = "atomic_regression_cc_rust",
    cpp_target = ":atomic_regression_cc",
)

crubit_rust_test(
    name = "atomic_regression_test",
    srcs = ["atomic_regression.rs"],
    deps = [
        ":atomic_regression_cc_rust",
        "@crate_index//:googletest",
    ],
)
