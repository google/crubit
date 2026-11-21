load("@rules_rust//rust:defs.bzl", "rust_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "method_exists",
    testonly = True,
    srcs = ["method_exists.rs"],
    edition = "2024",
)

crubit_test_cc_library(
    name = "failed_template_instantiation_member_function",
    hdrs = ["failed_template_instantiation_member_function.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "failed_template_instantiation_member_function_rust",
    cpp_target = ":failed_template_instantiation_member_function",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":failed_template_instantiation_member_function_rust",
        ":method_exists",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "failed_template_instantiation_member_function_recursive",
    hdrs = ["failed_template_instantiation_member_function_recursive.h"],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "failed_template_instantiation_member_function_recursive_rust",
    cpp_target = ":failed_template_instantiation_member_function_recursive",
)

crubit_rust_test(
    name = "test_recursive",
    srcs = ["test_recursive.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":failed_template_instantiation_member_function_recursive_rust",
        ":method_exists",
        "@crate_index//:googletest",
    ],
)
