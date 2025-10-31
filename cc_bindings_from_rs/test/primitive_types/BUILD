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
    name = "primitive_types",
    testonly = 1,
    srcs = ["primitive_types.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = ["@crate_index//:libc"],  # v0_2
)

cc_bindings_from_rust(
    name = "primitive_types_cc_api",
    testonly = 1,
    crate = ":primitive_types",
)

golden_test(
    name = "primitive_types_golden_test",
    basename = "primitive_types",
    golden_h = "primitive_types_cc_api.h",
    golden_rs = "primitive_types_cc_api_impl.rs",
    rust_library = "primitive_types",
)

crubit_cc_test(
    name = "primitive_types_test",
    srcs = ["primitive_types_test.cc"],
    deps = [
        ":primitive_types_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
