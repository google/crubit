load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "two",
    hdrs = ["two.h"],
)

crubit_test_cc_library(
    name = "three",
    hdrs = ["three.h"],
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
