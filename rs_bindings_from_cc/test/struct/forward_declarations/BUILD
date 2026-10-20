"""End-to-end test of forward declarations."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition",
    srcs = ["definition.cc"],
    hdrs = ["definition.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "definition_rust",
    cpp_target = ":definition",
)

crubit_test_cc_library(
    name = "declaration_1",
    hdrs = ["declaration_1.h"],
    aspect_hints = ["//features:forward_declare"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

rust_api_from_cpp(
    name = "declaration_1_rust",
    cpp_target = ":declaration_1",
)

crubit_test_cc_library(
    name = "declaration_2",
    hdrs = ["declaration_2.h"],
    aspect_hints = ["//features:forward_declare"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

rust_api_from_cpp(
    name = "declaration_2_rust",
    cpp_target = ":declaration_2",
)

crubit_test_cc_library(
    name = "declaration_other",
    hdrs = ["declaration_other.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "declaration_other_rust",
    cpp_target = ":declaration_other",
)

crubit_test_cc_library(
    name = "no_definition_in_headers",
    hdrs = ["no_definition_in_headers.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "no_definition_in_headers_rust",
    cpp_target = ":no_definition_in_headers",
)

crubit_rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    deps = [
        ":declaration_1_rust",
        ":declaration_2_rust",
        ":declaration_other_rust",
        ":definition_rust",
        ":no_definition_in_headers_rust",
        "//support:cref",
        "//support:ctor",
        "//support:forward_declare",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
