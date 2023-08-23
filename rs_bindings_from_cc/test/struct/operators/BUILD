"""End-to-end test for operators."""

load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "add",
    srcs = ["add.cc"],
    hdrs = ["add.h"],
)

crubit_rust_test(
    name = "add_test",
    srcs = ["add_test.rs"],
    cc_deps = [":add"],
    deps = [
        "//support:ctor",
    ],
)

crubit_test_cc_library(
    name = "add_assign",
    srcs = ["add_assign.cc"],
    hdrs = ["add_assign.h"],
)

crubit_rust_test(
    name = "add_assign_test",
    srcs = ["add_assign_test.rs"],
    cc_deps = [":add_assign"],
    deps = [
        "//support:ctor",
        "@crate_index//:static_assertions",
    ],
)

crubit_test_cc_library(
    name = "operators",
    srcs = ["operators.cc"],
    hdrs = ["operators.h"],
)

crubit_rust_test(
    name = "operators_test",
    srcs = ["operators_test.rs"],
    cc_deps = [":operators"],
    deps = [
        "//support:ctor",
        "@crate_index//:static_assertions",
    ],
)
