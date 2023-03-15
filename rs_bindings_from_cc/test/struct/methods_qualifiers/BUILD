load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "methods_qualifiers",
    srcs = ["methods_qualifiers.cc"],
    hdrs = ["methods_qualifiers.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":methods_qualifiers"],
    deps = [
        "//support:ctor",
    ],
)
