"""End-to-end test of forward declarations."""


load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition",
    srcs = ["definition.cc"],
    hdrs = ["definition.h"],
)

crubit_test_cc_library(
    name = "declaration_1",
    hdrs = ["declaration_1.h"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

crubit_test_cc_library(
    name = "declaration_2",
    hdrs = ["declaration_2.h"],
    deps = [
        ":definition",  # build_cleaner: keep
    ],
)

crubit_test_cc_library(
    name = "no_definition_in_headers",
    hdrs = ["no_definition_in_headers.h"],
)

rust_test(
    name = "forward_declarations_test",
    srcs = ["forward_declarations_test.rs"],
    cc_deps = [
        ":declaration_1",
        ":declaration_2",
        ":definition",
        ":no_definition_in_headers",
    ],
    deps = [
        "//support:ctor",
        "//support:forward_declare",
    ],
)
