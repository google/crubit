load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

crubit_test_cc_library(
    name = "calls_back_to_rust",
    srcs = ["calls_back_to_rust.cc"],
    hdrs = ["calls_back_to_rust.h"],
    deps = [
        "//support:annotations",
        "//support/rs_std:dyn_callable",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":calls_back_to_rust"],
    deps = [
        "@crate_index//:googletest",
    ],
)
