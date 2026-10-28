load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "crate_derive",
    hdrs = ["crate_derive.h"],
    aspect_hints = [
        "//features:record_impl_debug",
    ],
    deps = [
        "//support/internal:bindings_support",
    ],
)

rust_api_from_cpp(
    name = "crate_derive_rust",
    cpp_target = ":crate_derive",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":crate_derive_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
