"""End-to-end tests of `cc_bindings_from_rs`, focusing on unit-struct-related
bindings."""

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
    name = "unit_structs",
    testonly = 1,
    srcs = ["unit_structs.rs"],
)

golden_test(
    name = "unit_structs_golden_test",
    basename = "unit_structs",
    golden_h = "unit_structs_cc_api.h",
    golden_rs = "unit_structs_cc_api_impl.rs",
    rust_library = "unit_structs",
)

cc_bindings_from_rust(
    name = "unit_structs_cc_api",
    testonly = 1,
    crate = ":unit_structs",
)

crubit_cc_test(
    name = "unit_structs_test",
    srcs = ["unit_structs_test.cc"],
    deps = [
        ":unit_structs_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
