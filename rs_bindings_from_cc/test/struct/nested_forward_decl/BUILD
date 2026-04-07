load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nested_forward_decl",
    hdrs = ["nested_forward_decl.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "test_inner_exists",
    srcs = ["test_inner_exists.rs"],
    cc_deps = [":nested_forward_decl"],
    deps = ["@crate_index//:googletest"],
)
