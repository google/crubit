load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "reopened_namespace",
    hdrs = ["reopened_namespace.h"],
    deps = [":original_namespace"],
)

rust_api_from_cpp(
    name = "reopened_namespace_rust",
    cpp_target = ":reopened_namespace",
)

crubit_test_cc_library(
    name = "original_namespace",
    hdrs = ["original_namespace.h"],
    # disable Crubit
    aspect_hints = [],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":reopened_namespace_rust",
        "@crate_index//:googletest",
    ],
)
