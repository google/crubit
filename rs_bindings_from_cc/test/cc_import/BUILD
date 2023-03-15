load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)

package(default_applicable_licenses = ["//third_party/crubit:license"])

cc_library(
    name = "two",
    hdrs = ["two.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "three",
    hdrs = ["three.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

rust_library(
    name = "math",
    srcs = ["math.rs"],
    cc_deps = [
        ":three",
        ":two",
    ],
    proc_macro_deps = [
        "//support/cc_import",
    ],
)

rust_test(
    name = "math_test",
    crate = ":math",
)
