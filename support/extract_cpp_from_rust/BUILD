load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "parse_inline_cpp_macros",
    srcs = ["parse_inline_cpp_macros.rs"],
    visibility = ["//visibility:public"],
    deps = [
        "@crate_index//:ra_ap_rustc_lexer",  # v0_165
    ],
)

rust_test(
    name = "parse_inline_cpp_macros_test",
    crate = ":parse_inline_cpp_macros",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_binary(
    name = "extract_cpp_from_rust",
    srcs = ["extract_cpp_from_rust.rs"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":inline_cpp_utils",
        ":parse_inline_cpp_macros",
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

exports_files(
    [
        "inline_cpp_macro.rs",
        "inline_cpp_utils.rs",
    ],
    visibility = ["//support:__pkg__"],
)

alias(
    name = "inline_cpp_utils",
    actual = "//support:inline_cpp_utils",
    visibility = ["//visibility:private"],
)

alias(
    name = "inline_cpp_macro",
    actual = "//support:inline_cpp_macro",
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "rust_with_cpp_fmt",
    srcs = ["rust_with_cpp_fmt.rs"],
    visibility = ["//visibility:public"],
    deps = [
        ":parse_inline_cpp_macros",
        "//common:external_binaries",
        "@crate_index//:anyhow",  # v1
        "@crate_index//:clap",  # v4
        "@crate_index//:ra_ap_rustc_lexer",  # v0_165
    ],
)

crubit_rust_test(
    name = "rust_with_cpp_fmt_test",
    crate = ":rust_with_cpp_fmt",
    data = [
    ],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
