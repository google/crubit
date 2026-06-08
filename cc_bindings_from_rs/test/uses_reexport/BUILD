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
    name = "uses_reexport",
    testonly = 1,
    srcs = ["uses_reexport.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "uses_reexport_cc_api",
    testonly = 1,
    crate = ":uses_reexport",
)

golden_test(
    name = "uses_reexport_golden_test",
    basename = "uses_reexport",
    golden_h = "uses_reexport_cc_api.h",
    golden_rs = "uses_reexport_cc_api_impl.rs",
    rust_library = "uses_reexport",
)

crubit_cc_test(
    name = "uses_reexport_test",
    srcs = ["uses_reexport_test.cc"],
    deps = [
        ":uses_reexport_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
