"""End-to-end example of using type aliases that refer to fully-instantiated
templates in a different target (where the target with the header doesn't
have this particular instantiation)."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "template_without_instantiation",
    hdrs = ["template_without_instantiation.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

crubit_test_cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    aspect_hints = ["//features:supported"],
    deps = [":template_without_instantiation"],
)

rust_api_from_cpp(
    name = "type_alias_in_different_target_rust",
    cpp_target = ":type_alias_in_different_target",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":type_alias_in_different_target_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
