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
    name = "never",
    srcs = ["never.rs"],
)

golden_test(
    name = "never_golden_test",
    basename = "never",
    golden_h = "never_cc_api.h",
    golden_rs = "never_cc_api_impl.rs",
    rust_library = "never",
)

cc_bindings_from_rust(
    name = "never_cc_api",
    testonly = 1,
    crate = ":never",
)

crubit_cc_test(
    name = "never_test",
    srcs = ["never_test.cc"],
    deps = [
        ":never_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
