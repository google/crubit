"""End-to-end example of using functions with special names."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "special_naming",
    srcs = ["special_naming.cc"],
    hdrs = ["special_naming.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "special_naming_rust",
    cpp_target = ":special_naming",
)

golden_test(
    name = "special_naming_golden_test",
    basename = "special_naming",
    cc_library = "special_naming",
    golden_cc = "special_naming_api_impl.cc",
    golden_rs = "special_naming_rs_api.rs",
)

crubit_rust_test(
    name = "special_naming_test",
    srcs = ["test.rs"],
    deps = [
        ":special_naming_rust",
        "@crate_index//:googletest",
    ],
)
