load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    rust_golden_test = "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
# TODO(jeanpierreda): Uncomment this, see below.
# load(
#     "//rs_bindings_from_cc/test/golden:golden_test.bzl",
#     cc_golden_test = "golden_test",
# )

rust_library(
    name = "leaf_rs_lib",
    srcs = ["leaf_rs_lib.rs"],
    aspect_hints = ["//features:supported"],
)

rust_golden_test(
    name = "leaf_rs_lib_golden_test",
    basename = "leaf_rs_lib",
    golden_h = "leaf_rs_lib_cc_api.h",
    rust_library = "leaf_rs_lib",
)

cc_bindings_from_rust(
    name = "leaf_rs_lib_cc_api",
    crate = ":leaf_rs_lib",
)

crubit_test_cc_library(
    name = "middle_cc_lib",
    hdrs = ["middle_cc_lib.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        ":leaf_rs_lib_cc_api",
    ],
)

# TODO(jeanpierreda): Currently puts bindings from deps in, causing leeks, and exposes temporary
# output paths.
#
# cc_golden_test(
#     name = "middle_cc_lib_golden_test",
#     basename = "middle_cc_lib",
#     cc_library = "middle_cc_lib",
#     golden_rs = "middle_cc_lib_rs_api.rs",
# )

crubit_rust_test(
    name = "rs_test",
    srcs = ["rs_test.rs"],
    cc_deps = [
        ":middle_cc_lib",
    ],
    deps = [
        ":leaf_rs_lib",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "leaf_cc_lib",
    hdrs = ["leaf_cc_lib.h"],
    aspect_hints = ["//features:supported"],
)

rust_library(
    name = "middle_rs_lib",
    srcs = ["middle_rs_lib.rs"],
    aspect_hints = ["//features:supported"],
    cc_deps = [":leaf_cc_lib"],
)

cc_bindings_from_rust(
    name = "middle_rs_lib_cc_api",
    crate = ":middle_rs_lib",
)

crubit_cc_test(
    name = "cc_test",
    srcs = ["cc_test.cc"],
    deps = [
        ":leaf_cc_lib",
        ":middle_rs_lib_cc_api",
        "@googletest//:gtest_main",
    ],
)
