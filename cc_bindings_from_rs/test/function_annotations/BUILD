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
    name = "function_annotations",
    testonly = 1,
    srcs = ["function_annotations.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "function_annotations_cc_api",
    testonly = 1,
    crate = ":function_annotations",
)

golden_test(
    name = "function_annotations_golden_test",
    basename = "function_annotations",
    golden_h = "function_annotations_cc_api.h",
    golden_rs = "function_annotations_cc_api_impl.rs",
    kythe_annotations = True,
    rust_library = "function_annotations",
)

crubit_cc_test(
    name = "function_annotations_test",
    srcs = ["function_annotations_test.cc"],
    deps = [
        ":function_annotations_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
