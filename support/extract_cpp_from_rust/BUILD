load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)

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
    harness = "libtest",
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
    harness = "libtest",
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
