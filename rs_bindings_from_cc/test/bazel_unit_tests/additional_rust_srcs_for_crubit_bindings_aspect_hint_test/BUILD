load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(":additional_rust_srcs_for_crubit_bindings_aspect_hint_test.bzl", "additional_rust_srcs_for_crubit_bindings_aspect_hint_test_suite")

licenses(["notice"])

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

rust_api_from_cpp(
    name = "empty_cc_lib_with_additional_rust_srcs_rust",
    cpp_target = ":empty_cc_lib_with_additional_rust_srcs",
)

crubit_rust_test(
    name = "additional_rust_srcs_test",
    srcs = ["test.rs"],
    deps = [
        ":empty_cc_lib_with_additional_rust_srcs_rust",
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
    cc_deps = [":empty_cc_lib_with_additional_rust_srcs"],
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

rust_api_from_cpp(
    name = "cc_lib_with_additional_rust_srcs_with_namespace_path_rust",
    cpp_target = ":cc_lib_with_additional_rust_srcs_with_namespace_path",
)

crubit_rust_test(
    name = "additional_rust_srcs_with_namespace_path_test",
    srcs = ["additional_rust_srcs_with_namespace_path_test.rs"],
    deps = [
        ":cc_lib_with_additional_rust_srcs_with_namespace_path_rust",
        "@crate_index//:googletest",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_rust_srcs_try_trait",
    srcs = [
        "try_trait_stub.rs",
    ],
    unstable_rust_features = ["try_trait_v2"],
)

crubit_test_cc_library(
    name = "cc_lib_with_additional_rust_srcs_try_trait",
    hdrs = ["empty.h"],
    aspect_hints = [
        ":additional_rust_srcs_try_trait",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "cc_lib_with_additional_rust_srcs_try_trait_rust",
    cpp_target = ":cc_lib_with_additional_rust_srcs_try_trait",
)

crubit_rust_test(
    name = "additional_rust_srcs_try_trait_test",
    srcs = ["try_trait_test.rs"],
    deps = [
        ":cc_lib_with_additional_rust_srcs_try_trait_rust",
        "@crate_index//:googletest",
    ],
)
