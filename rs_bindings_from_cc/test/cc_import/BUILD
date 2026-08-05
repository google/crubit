load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "two",
    hdrs = ["two.h"],
    aspect_hints = [":two_rust.hint"],
)

crubit_test_cc_library(
    name = "three",
    hdrs = ["three.h"],
    aspect_hints = [":three_rust.hint"],
)

rust_library(
    name = "math",
    srcs = ["math.rs"],
    proc_macro_deps = [
        "//support/cc_import",
    ],
    deps = [
        ":three_rust",
        ":two_rust",
    ],
)

crubit_rust_test(
    name = "math_test",
    crate = ":math",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_api_from_cpp(
    name = "three_rust",
    cpp_target = "//rs_bindings_from_cc/test/cc_import:three",
)

rust_api_from_cpp(
    name = "two_rust",
    cpp_target = "//rs_bindings_from_cc/test/cc_import:two",
)
