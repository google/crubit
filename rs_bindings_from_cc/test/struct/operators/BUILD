"""End-to-end test for operators."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "add",
    srcs = ["add.cc"],
    hdrs = ["add.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "add_test",
    srcs = ["add_test.rs"],
    cc_deps = [":add"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "add_assign",
    srcs = ["add_assign.cc"],
    hdrs = ["add_assign.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "add_assign_test",
    srcs = ["add_assign_test.rs"],
    cc_deps = [":add_assign"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)

crubit_test_cc_library(
    name = "rename",
    hdrs = ["rename.h"],
    aspect_hints = ["//features:experimental"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "rename_test",
    srcs = ["rename_test.rs"],
    cc_deps = [":rename"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "operators",
    srcs = ["operators.cc"],
    hdrs = ["operators.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "operators_test",
    srcs = ["operators_test.rs"],
    cc_deps = [":operators"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
