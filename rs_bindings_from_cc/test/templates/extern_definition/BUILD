"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "extern_definition",
    srcs = ["extern_definition.cc"],
    hdrs = ["extern_definition.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "extern_definition_rust",
    cpp_target = ":extern_definition",
)

crubit_test_cc_library(
    name = "actual_instantiation",
    hdrs = ["actual_instantiation.h"],
    aspect_hints = ["//features:supported"],
    deps = [":extern_definition"],
)

rust_api_from_cpp(
    name = "actual_instantiation_rust",
    cpp_target = ":actual_instantiation",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":actual_instantiation_rust",
        ":extern_definition_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
