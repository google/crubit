load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "thread_safe_types",
    hdrs = ["thread_safe_types.h"],
)

crubit_test_cc_library(
    name = "thread_unsafe_types",
    hdrs = ["thread_unsafe_types.h"],
)

crubit_rust_test(
    name = "thread_safety",
    srcs = ["test.rs"],
    cc_deps = [
        ":thread_safe_types",
        ":thread_unsafe_types",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
