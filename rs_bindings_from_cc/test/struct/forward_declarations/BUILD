"""End-to-end test of forward declarations."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition",
    srcs = ["definition.cc"],
    hdrs = ["definition.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_test_cc_library(
    name = "declaration_1",
    hdrs = ["declaration_1.h"],
    aspect_hints = ["//features:experimental"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

crubit_test_cc_library(
    name = "declaration_2",
    hdrs = ["declaration_2.h"],
    aspect_hints = ["//features:experimental"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

crubit_test_cc_library(
    name = "declaration_other",
    hdrs = ["declaration_other.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_test_cc_library(
    name = "no_definition_in_headers",
    hdrs = ["no_definition_in_headers.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    cc_deps = [
        ":declaration_1",
        ":declaration_2",
        ":declaration_other",
        ":definition",
        ":no_definition_in_headers",
    ],
    deps = [
        "//support:ctor",
        "//support:forward_declare",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
