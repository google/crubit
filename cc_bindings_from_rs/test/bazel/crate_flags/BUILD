load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "crate_flags",
    testonly = 1,
    srcs = ["crate_flags.rs"],
    rustc_flags = [
        # Features specified in rustc_flags should also propagate to Crubit,
        # not only the ones specified in `crate_features`.
        "--cfg=feature=\"required_feature\"",
        # -Dunsafe-code is fine on the target, and should not cause bindings to fail.
        "-Dunsafe-code",
    ],
)

cc_bindings_from_rust(
    name = "crate_flags_cc_api",
    testonly = 1,
    crate = ":crate_flags",
)

crubit_cc_test(
    name = "crate_flags_test",
    srcs = ["crate_flags_test.cc"],
    deps = [
        ":crate_flags_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
