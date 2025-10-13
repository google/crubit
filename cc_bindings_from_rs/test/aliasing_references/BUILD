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
    name = "aliasing_references",
    srcs = ["aliasing_references.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "aliasing_references_golden_test",
    basename = "aliasing_references",
    golden_h = "aliasing_references_cc_api.h",
    golden_rs = "aliasing_references_cc_api_impl.rs",
    rust_library = "aliasing_references",
)

cc_bindings_from_rust(
    name = "aliasing_references_cc_api",
    testonly = 1,
    crate = ":aliasing_references",
)

crubit_cc_test(
    name = "aliasing_references_test",
    srcs = ["aliasing_references_test.cc"],
    deps = [
        ":aliasing_references_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
