"""End-to-end tests of `cc_bindings_from_rs`, focusing on union-related
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
    name = "unions",
    testonly = 1,
    srcs = ["unions.rs"],

    # TODO(b/262878759): Remove this once `all` is not automatically added anymore.
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "unions_golden_test",
    basename = "unions",
    golden_h = "unions_cc_api.h",
    golden_rs = "unions_cc_api_impl.rs",
    rust_library = "unions",
)

cc_bindings_from_rust(
    name = "unions_cc_api",
    testonly = 1,
    crate = ":unions",
)

crubit_cc_test(
    name = "unions_test",
    srcs = ["unions_test.cc"],
    deps = [
        ":unions_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
