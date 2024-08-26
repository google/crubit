load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/fission_build:enable_fission.bzl", "enable_fission_test")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "empty",
    hdrs = ["empty.h"],
    # With this option Clang doesn't produce a `.dwo` file with --fission; a wrapper script
    # takes care of creating an empty `.dwo` file to satisfy Bazel's expectations.
    copts = ["-g0"],
)

crubit_rust_test(
    name = "rust_test",
    srcs = ["empty.rs"],
    cc_deps = [
        ":empty",
    ],
    # Do not introduce a fake dependency on Crubit, because it adds conflicting actions.
    crubit_dep = False,
)

# Regression test for b/326976757.
enable_fission_test(
    name = "dwo_is_not_created_test",
    target_under_test = ":rust_test",
)
