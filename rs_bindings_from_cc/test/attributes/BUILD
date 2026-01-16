load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

cc_library(
    name = "annotalysis",
    hdrs = ["annotalysis.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/base:core_headers",
    ],
)

crubit_rust_test(
    name = "annotalysis_test",
    srcs = ["annotalysis_test.rs"],
    cc_deps = [":annotalysis"],
    deps = [
        "@crate_index//:googletest",
    ],
)
