"""End-to-end example of using simple functions."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "simple",
    srcs = ["simple.cc"],
    hdrs = ["simple.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

golden_test(
    name = "simple_golden_test",
    basename = "simple",
    cc_library = "simple",
    golden_cc = "simple_api_impl.cc",
    golden_rs = "simple_rs_api.rs",
)

crubit_rust_test(
    name = "simple_test",
    srcs = ["simple_test.rs"],
    cc_deps = [":simple"],
    deps = [
        "@crate_index//:googletest",
    ],
)
