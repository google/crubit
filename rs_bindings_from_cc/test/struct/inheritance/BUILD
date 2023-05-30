"""End-to-end test of inheritance."""


load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "upcast",
    hdrs = ["upcast.h"],
)

rust_test(
    name = "upcast_test",
    srcs = ["upcast_test.rs"],
    cc_deps = [":upcast"],
    deps = [
        "//support:ctor",
        "//support:oops",
    ],
)
