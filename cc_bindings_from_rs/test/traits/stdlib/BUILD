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
    name = "stdlib",
    testonly = 1,
    srcs = ["stdlib.rs"],
)

cc_bindings_from_rust(
    name = "stdlib_cc_api",
    testonly = 1,
    crate = ":stdlib",
)

golden_test(
    name = "stdlib_golden_test",
    basename = "stdlib",
    golden_h = "stdlib_cc_api.h",
    golden_rs = "stdlib_cc_api_impl.rs",
    rust_library = "stdlib",
)

crubit_cc_test(
    name = "stdlib_test",
    srcs = ["stdlib_test.cc"],
    deps = [
        ":stdlib_cc_api",
        "//support/rs_std",
        "//support/rs_std:iterator_adapter",
        "//support/rs_std:rs_core",
        "//testing/base/public:gunit_main",
    ],
)
