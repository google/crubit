load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

crubit_test_cc_library(
    name = "displayables",
    hdrs = ["displayables.h"],
    aspect_hints = [
        "//features:fmt",
    ],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/strings:str_format",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":displayables"],
    deps = [
        "@crate_index//:googletest",
    ],
)
