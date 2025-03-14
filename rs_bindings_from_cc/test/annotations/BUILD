load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "must_bind",
    hdrs = ["must_bind.h"],
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

crubit_rust_test(
    name = "must_bind_test",
    srcs = ["must_bind_test.rs"],
    cc_deps = [
        ":must_bind",
    ],
)
