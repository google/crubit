"""End-to-end tests of `cc_bindings_from_rs`, focusing on use statements."""

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
    name = "extern_crate",
    testonly = 1,
    srcs = ["extern_crate.rs"],
)

rust_library(
    name = "uses",
    testonly = 1,
    srcs = ["uses.rs"],
    proc_macro_deps = ["//support:crubit_annotate"],
    deps = [":extern_crate"],
)

cc_bindings_from_rust(
    name = "uses_cc_api",
    testonly = 1,
    crate = ":uses",
)

crubit_cc_test(
    name = "uses_test",
    srcs = ["uses_test.cc"],
    deps = [
        ":uses_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

golden_test(
    name = "uses_golden_test",
    basename = "uses",
    golden_h = "uses_cc_api.h",
    golden_rs = "uses_cc_api_impl.rs",
    rust_library = "uses",
)
