"""End-to-end tests of `cc_bindings_from_rs`, focusing on consts-related
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
    name = "consts",
    testonly = 1,
    srcs = ["consts.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        "//support/ffi_11",
    ],
)

golden_test(
    name = "consts_golden_test",
    basename = "consts",
    golden_h = "consts_cc_api.h",
    golden_rs = "consts_cc_api_impl.rs",
    rust_library = "consts",
)

cc_bindings_from_rust(
    name = "consts_cc_api",
    testonly = 1,
    crate = ":consts",
)

crubit_cc_test(
    name = "consts_test",
    srcs = ["consts_test.cc"],
    deps = [
        ":consts_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
