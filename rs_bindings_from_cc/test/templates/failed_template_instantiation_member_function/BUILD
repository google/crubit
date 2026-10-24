load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

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
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "failed_template_instantiation_member_function_recursive",
    hdrs = ["failed_template_instantiation_member_function_recursive.h"],
    tags = [
        # TODO(b/248542210): Enable when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "failed_template_instantiation_member_function_recursive_rust",
    cpp_target = ":failed_template_instantiation_member_function_recursive",
    tags = [
        # TODO(b/248542210): Enable when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
)

crubit_rust_test(
    name = "test_recursive",
    srcs = ["test_recursive.rs"],
    tags = [
        # TODO(b/248542210): Enable when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":failed_template_instantiation_member_function_recursive_rust",
        "@crate_index//:googletest",
    ],
)
