load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/bazel/fission_build:enable_fission.bzl", "enable_fission_test")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "empty",
    hdrs = ["empty.h"],
    # With this option Clang doesn't produce a `.dwo` file with --fission; a wrapper script
    # takes care of creating an empty `.dwo` file to satisfy Bazel's expectations.
    copts = ["-g0"],
)

rust_api_from_cpp(
    name = "empty_rust",
    cpp_target = ":empty",
)

crubit_rust_test(
    name = "rust_test",
    srcs = ["empty.rs"],
    deps = [
        ":empty_rust",
    ],
)

# Regression test for b/326976757.
enable_fission_test(
    name = "dwo_is_not_created_test",
    target_under_test = ":rust_test",
)
