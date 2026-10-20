"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "class_template_definition",
    hdrs = ["class_template_definition.h"],
    aspect_hints = [
        "//features:forward_declare",
        "//features:template_instantiation",
    ],
)

crubit_test_cc_library(
    name = "class_template_instantiation1",
    hdrs = ["class_template_instantiation1.h"],
    aspect_hints = [
        "//features:forward_declare",
        "//features:template_instantiation",
    ],
    deps = [":class_template_definition"],
)

rust_api_from_cpp(
    name = "class_template_instantiation1_rust",
    cpp_target = ":class_template_instantiation1",
)

crubit_test_cc_library(
    name = "class_template_instantiation2",
    hdrs = ["class_template_instantiation2.h"],
    aspect_hints = [
        "//features:forward_declare",
        "//features:template_instantiation",
    ],
    deps = [":class_template_definition"],
)

rust_api_from_cpp(
    name = "class_template_instantiation2_rust",
    cpp_target = ":class_template_instantiation2",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        ":class_template_instantiation1_rust",
        ":class_template_instantiation2_rust",
        "//support:forward_declare",
        "@crate_index//:googletest",
    ],
)
