load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
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
    edition = "2024",
)

golden_test(
    name = "pass_by_value_unmovable_golden_test",
    basename = "pass_by_value_unmovable",
    golden_h = "pass_by_value_unmovable_cc_api.h",
    golden_rs = "pass_by_value_unmovable_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "pass_by_value_unmovable",
)

cpp_api_from_rust(
    name = "pass_by_value_unmovable_cc_api",
    testonly = 1,
    crate = ":pass_by_value_unmovable",
)
