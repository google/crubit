"""End-to-end example of using type aliases."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

rust_api_from_cpp(
    name = "type_alias_rust",
    cpp_target = ":type_alias",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":type_alias_rust",
        "@crate_index//:googletest",
    ],
)
