"""End-to-end tests of `cc_bindings_from_rs`, focusing on how
Bazel<->cc_bindings_from_rs integration handles generating bindings
for the `core` crate from the Rust standard library."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

crubit_cc_test(
    name = "rs_core_test",
    srcs = ["rs_core_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        "//support/rs_std:rs_core",
        "@googletest//:gtest_main",
    ],
)
