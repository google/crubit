load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "operator_and",
    hdrs = ["operator_and.h"],
    aspect_hints = ["//features:supported"],
)

golden_test(
    name = "operator_and_golden_test",
    basename = "operator_and",
    cc_library = "operator_and",
    golden_cc = "operator_and.golden.cc",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":operator_and"],
    deps = [
        "@crates//:googletest",
    ],
)
