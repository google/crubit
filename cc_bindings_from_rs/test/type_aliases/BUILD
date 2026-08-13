"""End-to-end tests of `cc_bindings_from_rs`, focusing on type aliases bindings."""

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
    name = "type_aliases",
    testonly = 1,
    srcs = ["type_aliases.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "type_aliases_cc_api",
    testonly = 1,
    crate = ":type_aliases",
)

golden_test(
    name = "type_aliases_golden_test",
    basename = "type_aliases",
    golden_h = "type_aliases_cc_api.h",
    golden_rs = "type_aliases_cc_api_impl.rs",
    platforms = ["android"],
    rust_library = "type_aliases",
)

crubit_cc_test(
    name = "type_aliases_test",
    srcs = ["type_aliases_test.cc"],
    platforms = ["android"],
    deps = [
        ":type_aliases_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
