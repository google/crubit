load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

licenses(["notice"])

crubit_test_cc_library(
    name = "never",
    srcs = ["never.cc"],
    hdrs = ["never.h"],
    deps = [
        "//support:annotations_internal",
        "//support/rs:never",
    ],
)

crubit_rust_test(
    name = "never_test",
    srcs = ["never_test.rs"],
    cc_deps = [":never"],
    deps = ["@crate_index//:googletest"],
)
