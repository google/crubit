load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

rust_library(
    name = "virtual_unique_ptr",
    testonly = 1,
    srcs = [
        "virtual_unique_ptr.rs",
    ],
    cc_deps = [
        ":test_helpers",
        "//support/public:cc_std",
    ],
)

cc_bindings_from_rust(
    name = "virtual_unique_ptr_cc_api",
    testonly = 1,
    crate = ":virtual_unique_ptr",
)

crubit_cc_test(
    name = "virtual_unique_ptr_test",
    srcs = ["virtual_unique_ptr_test.cc"],
    deps = [
        ":test_helpers",
        ":virtual_unique_ptr_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
