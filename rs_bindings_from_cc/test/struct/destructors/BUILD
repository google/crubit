"""End-to-end test for destructors."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "destructors",
    hdrs = ["destructors.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/base:nullability",
    ],
)

rust_api_from_cpp(
    name = "destructors_rust",
    cpp_target = ":destructors",
)

golden_test(
    name = "destructors_golden_test",
    basename = "destructors",
    cc_library = "destructors",
    golden_cc = "destructors_api_impl.cc",
    golden_rs = "destructors_rs_api.rs",
)

crubit_rust_test(
    name = "destructors_test",
    srcs = ["destructors_test.rs"],
    deps = [
        ":destructors_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
