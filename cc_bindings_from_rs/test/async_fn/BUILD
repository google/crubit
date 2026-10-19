load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "async_fn",
    testonly = 1,
    srcs = ["async_fn.rs"],
    aspect_hints = [
        ":async_fn_cc_api.hint",
        "//features:supported",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cpp_api_from_rust(
    name = "async_fn_cc_api",
    testonly = 1,
    crate = ":async_fn",
    extra_cc_hdrs = ["async_fn_existing_cpp_types.h"],
    namespace = "async_fn",
)

golden_test(
    name = "async_fn_golden_test",
    basename = "async_fn",
    custom_namespace = True,
    golden_h = "async_fn_cc_api.h",
    golden_rs = "async_fn_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "async_fn",
)

crubit_cc_test(
    name = "async_fn_test",
    srcs = ["async_fn_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":async_fn_cc_api",
        "//support:movable",
        "//support/internal:bindings_support",
        "//support/rs_std:waker",
        "@googletest//:gtest_main",
    ],
)
