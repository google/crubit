"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load("//rs_bindings_from_cc/test:crubit_rust_test.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "propagates_define",
    defines = ["DEFINE_FROM_DEPENDENCY"],
)

crubit_test_cc_library(
    name = "consumes_define",
    hdrs = ["consumes_define.h"],
    deps = [":propagates_define"],
)

crubit_rust_test(
    name = "calls_generator_on_consumes_define",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_define"],
)

cc_library(
    name = "sets_define",
    hdrs = ["sets_define.h"],
    defines = ["DEFINE_FROM_BUILD_FILE"],
)

crubit_rust_test(
    name = "calls_generator_on_sets_define",
    srcs = ["empty.rs"],
    cc_deps = [":sets_define"],
)

cc_library(
    name = "uses_copts",
    hdrs = ["uses_copts.h"],
    copts = ["-DDEFINE_FROM_COPTS"],
)

crubit_rust_test(
    name = "calls_generator_on_uses_copts",
    srcs = ["empty.rs"],
    cc_deps = [":uses_copts"],
)

cc_library(
    name = "consumes_header_from_includes",
    hdrs = ["consumes_header_from_includes.h"],
    deps = ["//nowhere/experimentstargets_for_testing:uses_includes"],
)

crubit_rust_test(
    name = "calls_generator_on_consumes_header_from_includes",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_header_from_includes"],
)

cc_library(
    name = "consumes_header_with_include_prefix",
    hdrs = ["consumes_header_with_include_prefix.h"],
    deps = ["//nowhere/experimentstargets_for_testing:uses_include_prefix"],
)

crubit_rust_test(
    name = "calls_generator_on_consumes_header_with_include_prefix",
    srcs = ["empty.rs"],
    cc_deps = [":consumes_header_with_include_prefix"],
)
