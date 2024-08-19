"""End-to-end example of using type aliases to fully-instantiated templates."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "class_template_definition",
    hdrs = ["class_template_definition.h"],
)

crubit_test_cc_library(
    name = "class_template_instantiation1",
    hdrs = ["class_template_instantiation1.h"],
    deps = [":class_template_definition"],
)

crubit_test_cc_library(
    name = "class_template_instantiation2",
    hdrs = ["class_template_instantiation2.h"],
    deps = [":class_template_definition"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":class_template_instantiation1",
        ":class_template_instantiation2",
    ],
    deps = [
        "//support:forward_declare",
        "//third_party/gtest_rust/googletest",
    ],
)
