load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "defines_macro",
    hdrs = ["defines_macro.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
)

cc_library(
    name = "defines_macro_that_uses_macro",
    hdrs = ["defines_macro_that_uses_macro.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":defines_macro"],
)

cc_library(
    name = "uses_macro",
    hdrs = ["uses_macro.h"],
    aspect_hints = ["//third_party/crubit:experimental"],
    deps = [":defines_macro_that_uses_macro"],
)

rust_test(
    name = "uses_struct_and_function_from_macro",
    srcs = ["uses_struct_and_function_from_macro.rs"],
    cc_deps = [
        ":uses_macro",
    ],
)
