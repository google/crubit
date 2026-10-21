"""End-to-end tests of `cc_bindings_from_rs`, focusing on type composable bridging."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

# load(
#     "//cc_bindings_from_rs/test/golden:golden_test.bzl",
#     "golden_test",
# )
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "composable_bridging",
    srcs = [
        "composable_bridging.rs",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        "//support:bridge_rust",
        "//support/cc_std",
    ],
)

# Disabled because the golden test generates third_party/crosstool/v18 includes.
# Instead, it should only generate includes for the libraries we actually rely on.
#
# golden_test(
#     name = "composable_bridging_golden_test",
#     basename = "composable_bridging",
#     golden_h = "composable_bridging_cc_api.h",
#     golden_rs = "composable_bridging_cc_api_impl.rs",
#     rust_library = "composable_bridging",
# )

cpp_api_from_rust(
    name = "composable_bridging_cc_api",
    crate = ":composable_bridging",
)

crubit_cc_test(
    name = "composable_bridging_test",
    srcs = ["composable_bridging_test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = ["no_windows"],
    deps = [
        ":composable_bridging_cc_api",
        "//support/rs_std:slice_ref",
        "@googletest//:gtest_main",
    ],
)
