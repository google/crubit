load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration1",
    hdrs = ["forward_declaration1.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "forward_declaration1_rust",
    cpp_target = ":forward_declaration1",
)

crubit_test_cc_library(
    name = "forward_declaration2",
    hdrs = ["forward_declaration2.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "forward_declaration2_rust",
    cpp_target = ":forward_declaration2",
)

crubit_test_cc_library(
    name = "definition",
    hdrs = ["definition.h"],
    aspect_hints = ["//features:forward_declare"],
)

rust_api_from_cpp(
    name = "definition_rust",
    cpp_target = ":definition",
)

# Test cases to ensure we can easily convert forward declarations of the same type across crates
# that don't depend on each other.
crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":definition_rust",
        ":forward_declaration1_rust",
        ":forward_declaration2_rust",
        "//support:forward_declare",
        "@crate_index//:googletest",
    ],
)
