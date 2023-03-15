# End-to-end test of !Unpin classes.

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "nonunpin",
    hdrs = ["nonunpin.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "nonunpin_test",
    srcs = ["nonunpin_test.rs"],
    cc_deps = [":nonunpin"],
    deps = ["//support:ctor"],
)
