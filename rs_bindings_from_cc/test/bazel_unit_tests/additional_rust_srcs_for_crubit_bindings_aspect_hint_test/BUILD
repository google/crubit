load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(":additional_rust_srcs_for_crubit_bindings_aspect_hint_test.bzl", "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite")

additional_rust_srcs_for_crubit_bindings(
    name = "additional_rust_srcs_test_stub",
    srcs = [
        "stub.rs",
    ],
)

crubit_test_cc_library(
    name = "empty_cc_lib_with_additional_rust_srcs",
    hdrs = ["empty.h"],
    aspect_hints = [
        ":additional_rust_srcs_test_stub",
    ],
)

crubit_rust_test(
    name = "additional_rust_srcs_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":empty_cc_lib_with_additional_rust_srcs",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite(
    name = "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite",
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_rust_srcs_with_namespace_path",
    srcs = [
        "inner.rs",
    ],
    namespace_path = "a::b::c",
)

crubit_test_cc_library(
    name = "cc_lib_with_additional_rust_srcs_with_namespace_path",
    hdrs = ["namespace_lib.h"],
    aspect_hints = [
        ":additional_rust_srcs_with_namespace_path",
        "//features:supported",
    ],
)

crubit_rust_test(
    name = "additional_rust_srcs_with_namespace_path_test",
    srcs = ["additional_rust_srcs_with_namespace_path_test.rs"],
    cc_deps = [
        ":cc_lib_with_additional_rust_srcs_with_namespace_path",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
