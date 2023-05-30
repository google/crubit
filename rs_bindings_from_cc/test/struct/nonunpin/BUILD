# End-to-end test of !Unpin classes.


load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nonunpin",
    hdrs = ["nonunpin.h"],
)

rust_test(
    name = "nonunpin_test",
    srcs = ["nonunpin_test.rs"],
    cc_deps = [":nonunpin"],
    deps = ["//support:ctor"],
)
