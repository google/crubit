"""End-to-end tests of `cc_bindings_from_rs`, focusing on struct-related
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
    name = "structs",
    testonly = 1,
    srcs = ["structs.rs"],

    # TODO(b/262878759): Remove this once `all` is not automatically added anymore.
    aspect_hints = [
        "//features:supported",
    ],
    proc_macro_deps = ["//support:crubit_annotate"],
)

golden_test(
    name = "structs_golden_test",
    basename = "structs",
    golden_h = "structs_cc_api.h",
    golden_rs = "structs_cc_api_impl.rs",
    rust_library = "structs",
)

cc_bindings_from_rust(
    name = "structs_cc_api",
    testonly = 1,
    crate = ":structs",
)

crubit_cc_test(
    name = "structs_test",
    srcs = ["structs_test.cc"],
    deps = [
        ":structs_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
