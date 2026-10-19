load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

crubit_cc_test(
    name = "core_bindings_test",
    srcs = ["core_bindings_test.cc"],
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
