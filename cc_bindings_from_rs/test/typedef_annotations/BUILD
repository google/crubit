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
    name = "typedef_annotations",
    testonly = 1,
    srcs = ["typedef_annotations.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "typedef_annotations_cc_api",
    testonly = 1,
    crate = ":typedef_annotations",
)

golden_test(
    name = "typedef_annotations_golden_test",
    basename = "typedef_annotations",
    golden_h = "typedef_annotations_cc_api.h",
    golden_rs = "typedef_annotations_cc_api_impl.rs",
    kythe_annotations = True,
    rust_library = "typedef_annotations",
)

crubit_cc_test(
    name = "typedef_annotations_test",
    srcs = ["typedef_annotations_test.cc"],
    deps = [
        ":typedef_annotations_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
