load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "failed_template_instantiation_member_function",
    hdrs = ["failed_template_instantiation_member_function.h"],
)

# A test that checks the setup `:failed_template_instantiation_member_function` somehow works.
crubit_cc_test(
    name = "failed_template_instantiation_member_function_cc_test",
    srcs = [
        "use_template_with_failable_member_function_test.cc",
    ],
    deps = [
        ":failed_template_instantiation_member_function",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":failed_template_instantiation_member_function",
    ],
)
