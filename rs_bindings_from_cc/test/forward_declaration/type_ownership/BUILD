load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    hdrs = ["forward_declaration.h"],
)

crubit_test_cc_library(
    name = "definition",
    hdrs = [
        "definition.h",
    ],
    deps = [
        ":forward_declaration",
    ],
)

# A test to check that this can actually happen in google3.
crubit_cc_test(
    name = "test_cc",
    srcs = [
        "test.cc",
    ],
    deps = [
        ":definition",
        ":forward_declaration",
        "@com_google_googletest//:gtest_main",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":definition",
        ":forward_declaration",
    ],
    tags = [
        # TODO(b/318690257): Enable when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
)

# The generated bindings for `definition.h` doesn't compile, but we can still import the header
# without generating bindings.
crubit_test_cc_library(
    name = "definition_with_crubit_disabled",
    hdrs = [
        "definition.h",
    ],
    aspect_hints = [],
    deps = [
        ":forward_declaration",
    ],
)

crubit_rust_test(
    name = "test_with_crubit_disabled",
    srcs = ["test_with_crubit_disabled.rs"],
    cc_deps = [
        ":definition_with_crubit_disabled",
    ],
)

crubit_test_cc_library(
    name = "forward_declaration_with_crubit_disabled",
    hdrs = ["forward_declaration.h"],
    aspect_hints = [],
)

crubit_test_cc_library(
    name = "definition_with_crubit_disabled_in_deps",
    hdrs = [
        "definition.h",
    ],
    aspect_hints = [],
    deps = [
        ":forward_declaration_with_crubit_disabled",
    ],
)

crubit_rust_test(
    name = "test_with_crubit_disabled_in_deps",
    srcs = ["test_with_crubit_disabled_in_deps.rs"],
    cc_deps = [
        ":definition_with_crubit_disabled_in_deps",
    ],
)
