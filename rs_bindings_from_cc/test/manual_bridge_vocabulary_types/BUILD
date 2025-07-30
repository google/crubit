load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

cc_library(
    name = "helper_lib",
    hdrs = ["helper.h"],
    aspect_hints = [
        "//features:experimental",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":helper_lib",
        "//support/cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
