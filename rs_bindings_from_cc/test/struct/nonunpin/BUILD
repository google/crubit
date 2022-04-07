# End-to-end test of !Unpin classes.

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "nonunpin",
    hdrs = ["nonunpin.h"],
)

rust_test(
    name = "nonunpin_test",
    srcs = ["nonunpin_test.rs"],
    cc_deps = [":nonunpin"],
    deps = ["//rs_bindings_from_cc/support:ctor"],
)
