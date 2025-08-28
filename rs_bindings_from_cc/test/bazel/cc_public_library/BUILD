load("@bazel_tools//tools/build_defs:cc_public_library.bzl", "cc_internal_library", "cc_public_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

additional_rust_srcs_for_crubit_bindings(
    name = "public_extra",
    srcs = ["public_extra.rs"],
)

cc_public_library(
    name = "public",
    srcs = ["public_src.h"],
    hdrs = ["public.h"],
    aspect_hints = [
        ":public_extra",
        "//features:supported",
    ],
    implementation = [":private"],
)

additional_rust_srcs_for_crubit_bindings(
    name = "private_extra",
    srcs = ["private_extra.rs"],
)

cc_internal_library(
    name = "private",
    srcs = ["private.h"],
    aspect_hints = [
        ":private_extra",
        "//features:supported",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":private",
        ":public",
    ],
)

cc_library(
    name = "ambiguous_header_rdep",
    hdrs = ["ambiguous_header_rdep.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":public",
        ":public_cc_public_library_headers",
    ],
)

crubit_rust_test(
    name = "ambiguous_header_rdep_test",
    srcs = ["ambiguous_header_rdep_test.rs"],
    cc_deps = [":ambiguous_header_rdep"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
