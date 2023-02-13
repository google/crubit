"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "propagates_define",
    defines = ["DEFINE_FROM_DEPENDENCY"],
)

cc_library(
    name = "consumes_define",
    hdrs = ["consumes_define.h"],
    deps = [":propagates_define"],
)

rust_test(
    name = "calls_generator_on_consumes_define",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_define"],
)

cc_library(
    name = "sets_define",
    hdrs = ["sets_define.h"],
    defines = ["DEFINE_FROM_BUILD_FILE"],
)

rust_test(
    name = "calls_generator_on_sets_define",
    srcs = ["empty.rs"],
    cc_deps = [":sets_define"],
)

cc_library(
    name = "uses_copts",
    hdrs = ["uses_copts.h"],
    copts = ["-DDEFINE_FROM_COPTS"],
)

rust_test(
    name = "calls_generator_on_uses_copts",
    srcs = ["empty.rs"],
    cc_deps = [":uses_copts"],
)

cc_library(
    name = "consumes_header_from_includes",
    hdrs = ["consumes_header_from_includes.h"],
    deps = ["//nowhere/experimentstargets_for_testing:uses_includes"],
)

rust_test(
    name = "calls_generator_on_consumes_header_from_includes",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_header_from_includes"],
)

cc_library(
    name = "consumes_header_with_include_prefix",
    hdrs = ["consumes_header_with_include_prefix.h"],
    deps = ["//nowhere/experimentstargets_for_testing:uses_include_prefix"],
)

rust_test(
    name = "calls_generator_on_consumes_header_with_include_prefix",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_header_with_include_prefix"],
)
