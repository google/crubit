"""End-to-end test for operators."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "add",
    srcs = ["add.cc"],
    hdrs = ["add.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "add_rust",
    cpp_target = ":add",
)

crubit_rust_test(
    name = "add_test",
    srcs = ["add_test.rs"],
    deps = [
        ":add_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "add_assign",
    srcs = ["add_assign.cc"],
    hdrs = ["add_assign.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "add_assign_rust",
    cpp_target = ":add_assign",
)

crubit_rust_test(
    name = "add_assign_test",
    srcs = ["add_assign_test.rs"],
    deps = [
        ":add_assign_rust",
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_test_cc_library(
    name = "rename",
    hdrs = ["rename.h"],
    aspect_hints = ["//features:supported"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "rename_rust",
    cpp_target = ":rename",
)

crubit_rust_test(
    name = "rename_test",
    srcs = ["rename_test.rs"],
    deps = [
        ":rename_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "foreign_lib",
    hdrs = ["foreign_lib.h"],
    aspect_hints = ["//features:supported"],
)

rust_api_from_cpp(
    name = "foreign_lib_rust",
    cpp_target = ":foreign_lib",
)

crubit_test_cc_library(
    name = "operators",
    srcs = ["operators.cc"],
    hdrs = ["operators.h"],
    aspect_hints = ["//features:supported"],
    deps = [":foreign_lib"],
)

rust_api_from_cpp(
    name = "operators_rust",
    cpp_target = ":operators",
)

crubit_rust_test(
    name = "operators_test",
    srcs = ["operators_test.rs"],
    deps = [
        ":foreign_lib_rust",
        ":operators_rust",
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_test_cc_library(
    name = "spaceship",
    hdrs = ["spaceship.h"],
    aspect_hints = ["//features:strong_ordering_traits"],
)

rust_api_from_cpp(
    name = "spaceship_rust",
    cpp_target = ":spaceship",
)

crubit_rust_test(
    name = "spaceship_test",
    srcs = ["spaceship_test.rs"],
    deps = [
        ":spaceship_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

# Crubit is intentionally not enabled for this library.
cc_library(
    name = "operator_mixins",
    hdrs = ["operator_mixins.h"],
)

crubit_test_cc_library(
    name = "inherited_operators",
    hdrs = ["inherited_operators.h"],
    aspect_hints = ["//features:inherited_friend_operators"],
    deps = [":operator_mixins"],
)

rust_api_from_cpp(
    name = "inherited_operators_rust",
    cpp_target = ":inherited_operators",
)

crubit_rust_test(
    name = "inherited_operators_test",
    srcs = ["inherited_operators_test.rs"],
    deps = [
        ":inherited_operators_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
