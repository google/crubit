load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "inline",
    hdrs = ["inline.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "inline_rust",
    cpp_target = ":inline",
)

golden_test(
    name = "inline_golden_test",
    basename = "inline",
    cc_library = "inline",
    golden_cc = "inline_api_impl.cc",
    golden_rs = "inline_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":inline_rust",
        "@crate_index//:googletest",
    ],
)
