load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

cc_library(
    name = "either",
    srcs = ["either_internal.h"],
    hdrs = ["either.h"],
    aspect_hints = [
        "//features:supported",
        ":additional_either_src",
    ],
    deps = [
        "//support:annotations",  # build_cleaner: keep
        "//support:bridge_cpp",  # build_cleaner: keep
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_either_src",
    srcs = ["additional_either_src.rs"],
    namespace_path = "either",
)

cc_library(
    name = "int_or_bool",
    srcs = ["int_or_bool.cc"],
    hdrs = ["int_or_bool.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [":either"],
)

crubit_rust_test(
    name = "rust_int_or_bool_test",
    srcs = ["rust_int_or_bool_test.rs"],
    cc_deps = [
        ":either",
        ":int_or_bool",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
