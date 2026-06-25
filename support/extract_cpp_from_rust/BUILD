load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)

package(default_applicable_licenses = ["//:license"])

rust_binary(
    name = "extract_cpp_from_rust",
    srcs = ["extract_cpp_from_rust.rs"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":inline_cpp_utils",
        "@crate_index//:clap",  # v4
        "@crate_index//:ra_ap_rustc_lexer",  # v0_165
    ],
)

rust_test(
    name = "extract_cpp_from_rust_test",
    crate = ":extract_cpp_from_rust",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "inline_cpp_utils",
    srcs = ["inline_cpp_utils.rs"],
    visibility = ["//visibility:private"],
)

rust_proc_macro(
    name = "inline_cpp_macro",
    srcs = ["inline_cpp_macro.rs"],
    visibility = ["//visibility:public"],
    deps = [
        ":inline_cpp_utils",
        "@crate_index//:quote",  # v1
    ],
)
