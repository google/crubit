"""End-to-end tests of `cc_bindings_from_rs`, focusing on tuple-struct-related
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
    name = "tuple_structs",
    testonly = 1,
    srcs = ["tuple_structs.rs"],

    # TODO(b/262878759): Remove this once `all` is not automatically added anymore.
    aspect_hints = [
        "//features:supported",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "tuple_structs_golden_test",
    basename = "tuple_structs",
    golden_h = "tuple_structs_cc_api.h",
    golden_rs = "tuple_structs_cc_api_impl.rs",
    rust_library = "tuple_structs",
)

cc_bindings_from_rust(
    name = "tuple_structs_cc_api",
    testonly = 1,
    crate = ":tuple_structs",
)

crubit_cc_test(
    name = "tuple_structs_test",
    srcs = ["tuple_structs_test.cc"],
    deps = [
        ":tuple_structs_cc_api",
        "@googletest//:gtest_main",
    ],
)
