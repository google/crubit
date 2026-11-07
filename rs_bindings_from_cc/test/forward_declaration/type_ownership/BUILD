load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    hdrs = ["forward_declaration.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "forward_declaration_rust",
    cpp_target = ":forward_declaration",
)

crubit_test_cc_library(
    name = "definition",
    hdrs = [
        "definition.h",
    ],
    aspect_hints = ["//features:supported"],
    deps = [
        ":forward_declaration",
    ],
)

rust_api_from_cpp(
    name = "definition_rust",
    cpp_target = ":definition",
)

crubit_cc_test(
    name = "test_cc",
    srcs = [
        "test.cc",
    ],
    platforms = ["android"],
    deps = [
        ":definition",
        ":forward_declaration",
        "@googletest//:gtest_main",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":definition_rust",
        ":forward_declaration_rust",
        "@crate_index//:googletest",
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
    deps = [
        ":definition_with_crubit_disabled",
        "@crate_index//:googletest",
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
    deps = [
        ":forward_declaration_with_crubit_disabled",
    ],
)

rust_api_from_cpp(
    name = "definition_with_crubit_disabled_in_deps_rust",
    cpp_target = ":definition_with_crubit_disabled_in_deps",
)

crubit_rust_test(
    name = "test_with_crubit_disabled_in_deps",
    srcs = ["test_with_crubit_disabled_in_deps.rs"],
    deps = [
        ":definition_with_crubit_disabled_in_deps_rust",
        "@crate_index//:googletest",
    ],
)
