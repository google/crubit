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
    name = "rust_api",
    testonly = 1,
    srcs = ["rust_api.rs"],
    features = ["crubit"],
    deps = [
        "//support/ffi_11",
    ],
)

cc_bindings_from_rust(
    name = "rust_api_cc_api",
    testonly = 1,
    crate = ":rust_api",
)

golden_test(
    name = "rust_api_golden_test",
    basename = "rust_api",
    golden_h = "rust_api_cc_api.h",
    rust_library = "rust_api",
)

crubit_cc_test(
    name = "type_identity_test",
    srcs = ["type_identity_test.cc"],
    deps = [
        ":rust_api_cc_api",
    ],
)
