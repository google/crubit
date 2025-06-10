load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "wrapper_library",
    hdrs = ["wrapper_library.h"],
    aspect_hints = [
        "//features:wrapper",
        ":wrapper_library_extra",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "wrapper_library_extra",
    srcs = ["wrapper_library_extra.rs"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":wrapper_library"],
    deps = [
        "@crate_index//:googletest",
    ],
)
