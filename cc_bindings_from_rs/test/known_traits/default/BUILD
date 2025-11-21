"""End-to-end tests of `cc_bindings_from_rs`, focusing on the `Default` trait"""

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

rust_library(
    # `default` is a reserved C++ keyword, so using `rs_` prefix.
    name = "rs_default",
    testonly = 1,
    srcs = ["default.rs"],
)

cc_bindings_from_rust(
    name = "rs_default_cc_api",
    testonly = 1,
    crate = ":rs_default",
)

golden_test(
    name = "default_golden_test",
    basename = "default",
    golden_h = "default_cc_api.h",
    golden_rs = "default_cc_api_impl.rs",
    rust_library = "rs_default",
)

crubit_cc_test(
    name = "default_test",
    srcs = ["default_test.cc"],
    deps = [
        ":rs_default_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
