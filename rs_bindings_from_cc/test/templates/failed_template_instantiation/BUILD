load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "failed_template_instantiation",
    hdrs = ["failed_template_instantiation.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "failed_template_instantiation_rust",
    cpp_target = ":failed_template_instantiation",
)

crubit_test_cc_library(
    name = "repeatedly_failed_template_instantiation",
    hdrs = ["repeatedly_failed_template_instantiation.h"],
    aspect_hints = ["//features:template_instantiation"],
)

rust_api_from_cpp(
    name = "repeatedly_failed_template_instantiation_rust",
    cpp_target = ":repeatedly_failed_template_instantiation",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":failed_template_instantiation_rust",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "test_repeated",
    srcs = ["test_repeated.rs"],
    deps = [
        ":repeatedly_failed_template_instantiation_rust",
        "@crate_index//:googletest",
    ],
)
