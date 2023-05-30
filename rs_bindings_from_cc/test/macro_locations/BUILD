
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "defines_macro",
    hdrs = ["defines_macro.h"],
)

crubit_test_cc_library(
    name = "defines_macro_that_uses_macro",
    hdrs = ["defines_macro_that_uses_macro.h"],
    deps = [":defines_macro"],
)

crubit_test_cc_library(
    name = "uses_macro",
    hdrs = ["uses_macro.h"],
    deps = [":defines_macro_that_uses_macro"],
)

rust_test(
    name = "uses_struct_and_function_from_macro",
    srcs = ["uses_struct_and_function_from_macro.rs"],
    cc_deps = [
        ":uses_macro",
    ],
)
