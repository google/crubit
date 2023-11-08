load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    hdrs = ["forward_declaration.h"],
)

crubit_test_cc_library(
    name = "definition_and_forward_declaration_same_cc_library",
    hdrs = [
        "definition.h",
        "forward_declaration.h",
    ],
)

crubit_test_cc_library(
    name = "definition_and_forward_declaration_separate_cc_library",
    hdrs = [
        "definition.h",
    ],
    deps = [
        ":forward_declaration",
    ],
)

crubit_test_cc_library(
    name = "forward_declaration2",
    hdrs = ["forward_declaration2.h"],
)

# Tests that Crubit doesn't crash when it sees forward declarations of the same struct in 2
# different targets.
crubit_test_cc_library(
    name = "definition_and_forward_declarations_separate_cc_library",
    hdrs = [
        "definition2.h",
    ],
    deps = [
        ":forward_declaration",
        ":forward_declaration2",
    ],
)

crubit_test_cc_library(
    name = "definition_and_forward_declarations_same_cc_library",
    hdrs = [
        "definition2.h",
        "forward_declaration.h",
        "forward_declaration2.h",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":definition_and_forward_declaration_same_cc_library",
        ":definition_and_forward_declaration_separate_cc_library",
        ":definition_and_forward_declarations_same_cc_library",
        ":definition_and_forward_declarations_separate_cc_library",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
)
