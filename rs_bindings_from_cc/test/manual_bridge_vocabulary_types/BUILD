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
    deps = [
        "@crate_index//:googletest",
    ],
)
