"""End-to-end example of using type aliases."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":type_alias"],
)
