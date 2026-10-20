load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "overloaded",
    hdrs = ["overloaded.h"],
)

rust_api_from_cpp(
    name = "overloaded_rust",
    cpp_target = ":overloaded",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":overloaded_rust",
        "@crate_index//:googletest",
    ],
)
