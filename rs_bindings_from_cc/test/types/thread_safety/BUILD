load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "thread_safe_types",
    hdrs = ["thread_safe_types.h"],
)

rust_api_from_cpp(
    name = "thread_safe_types_rust",
    cpp_target = ":thread_safe_types",
)

crubit_test_cc_library(
    name = "thread_unsafe_types",
    hdrs = ["thread_unsafe_types.h"],
)

rust_api_from_cpp(
    name = "thread_unsafe_types_rust",
    cpp_target = ":thread_unsafe_types",
)

crubit_rust_test(
    name = "thread_safety",
    srcs = ["test.rs"],
    deps = [
        ":thread_safe_types_rust",
        ":thread_unsafe_types_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
