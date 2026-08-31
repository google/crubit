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

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "pass_by_value_unmovable",
    testonly = 1,
    srcs = ["pass_by_value_unmovable.rs"],
)

golden_test(
    name = "pass_by_value_unmovable_golden_test",
    basename = "pass_by_value_unmovable",
    golden_h = "pass_by_value_unmovable_cc_api.h",
    golden_rs = "pass_by_value_unmovable_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "pass_by_value_unmovable",
)

cc_bindings_from_rust(
    name = "pass_by_value_unmovable_cc_api",
    testonly = 1,
    crate = ":pass_by_value_unmovable",
)
