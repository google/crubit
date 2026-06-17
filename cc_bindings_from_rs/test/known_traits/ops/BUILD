"""End-to-end tests of `cc_bindings_from_rs`, focusing on arithmetic and bitwise operators"""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "rs_ops",
    testonly = 1,
    srcs = ["rs_ops.rs"],
)

cc_bindings_from_rust(
    name = "rs_ops_cc_api",
    testonly = 1,
    crate = ":rs_ops",
)

golden_test(
    name = "rs_ops_cc_api_golden",
    basename = "rs_ops",
    golden_h = "rs_ops_cc_api.h",
    golden_rs = "rs_ops_cc_api_impl.rs",
    rust_library = "rs_ops",
)

crubit_cc_test(
    name = "rs_ops_test",
    srcs = ["rs_ops_test.cc"],
    deps = [
        ":rs_ops_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
