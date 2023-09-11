load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "methods_qualifiers",
    srcs = ["methods_qualifiers.cc"],
    hdrs = ["methods_qualifiers.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":methods_qualifiers"],
    deps = [
        "//support:ctor",
    ],
)
