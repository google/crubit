load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

licenses(["notice"])

cc_library(
    name = "string_test_lib",
    hdrs = ["string_test_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support/internal:bindings_support",  # build_cleaner: keep
    ],
)

crubit_rust_test(
    name = "string_test",
    srcs = ["string_test.rs"],
    cc_deps = [
        ":string_test_lib",
        "//support/cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
