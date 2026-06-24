load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_bindings_from_rust_library_config(
    name = "async_fn_bridged_type_hint",
    extra_cc_hdrs = ["async_fn_existing_cpp_types.h"],
)

rust_library(
    name = "async_fn",
    testonly = 1,
    srcs = ["async_fn.rs"],
    aspect_hints = [
        ":async_fn_bridged_type_hint",
        "//features:supported",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust(
    name = "async_fn_cc_api",
    testonly = 1,
    crate = ":async_fn",
)

golden_test(
    name = "async_fn_golden_test",
    basename = "async_fn",
    golden_h = "async_fn_cc_api.h",
    golden_rs = "async_fn_cc_api_impl.rs",
    rust_library = "async_fn",
)

crubit_cc_test(
    name = "async_fn_test",
    srcs = ["async_fn_test.cc"],
    deps = [
        ":async_fn_cc_api",
        "//support/public:bindings_support",
        "//support/rs_std:waker",
        "//testing/base/public:gunit_main",
    ],
)
