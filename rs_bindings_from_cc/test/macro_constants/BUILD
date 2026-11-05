load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(
    # Build and use precompiled header modules (where supported).
    features = ["header_modules"],
)

crubit_test_cc_library(
    name = "macro_constants",
    hdrs = ["macro_constants.h"],
    aspect_hints = ["//features:macro_constants"],
    textual_hdrs = ["macro_constants_textual.inc"],
)

rust_api_from_cpp(
    name = "macro_constants_rust",
    cpp_target = ":macro_constants",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":macro_constants_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
