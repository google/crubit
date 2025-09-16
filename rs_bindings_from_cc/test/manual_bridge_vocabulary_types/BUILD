load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

cc_library(
    name = "common",
    hdrs = ["common.h"],
    aspect_hints = [
        "//features:experimental",
    ],
)

cc_library(
    name = "unique_ptr_lib",
    hdrs = ["unique_ptr_lib.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":common",
    ],
)

crubit_rust_test(
    name = "unique_ptr_test",
    srcs = ["unique_ptr_test.rs"],
    cc_deps = [
        ":common",
        ":unique_ptr_lib",
        "//support/cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "vector_lib",
    hdrs = ["vector_lib.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":common",
    ],
)

crubit_rust_test(
    name = "vector_test",
    srcs = ["vector_test.rs"],
    cc_deps = [
        ":common",
        ":vector_lib",
        "//support/cc_std",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
