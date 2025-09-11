load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    srcs = [
        "forward_declaration.cc",
    ],
    hdrs = ["forward_declaration.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":forward_declaration",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
