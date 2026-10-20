load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:public"],
)

bzl_library(
    name = "cpp_api_from_rust_bzl",
    srcs = ["cpp_api_from_rust.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule_bzl",
    ],
)

bzl_library(
    name = "rust_api_from_cpp_bzl",
    srcs = ["rust_api_from_cpp.bzl"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp",
    ],
)

bzl_library(
    name = "additional_rust_srcs_for_crubit_bindings_allowlist_bzl",
    srcs = ["additional_rust_srcs_for_crubit_bindings_allowlist.bzl"],
)

bzl_library(
    name = "additional_rust_srcs_for_crubit_bindings_bzl",
    srcs = ["additional_rust_srcs_for_crubit_bindings.bzl"],
    deps = [
        ":additional_rust_srcs_for_crubit_bindings_allowlist_bzl",
        "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint_bzl",
    ],
)

bzl_library(
    name = "cpp_api_from_rust_library_config_bzl",
    srcs = ["cpp_api_from_rust_library_config.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint_bzl",
    ],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:providers_bzl",
        "//rs_bindings_from_cc/bazel_support:providers_bzl",
    ],
)

bzl_library(
    name = "rust_api_from_cpp_aspect_bzl",
    srcs = ["rust_api_from_cpp_aspect.bzl"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
    ],
)

bzl_library(
    name = "cpp_api_from_rust_aspect_bzl",
    srcs = ["cpp_api_from_rust_aspect.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule_bzl",
    ],
)

bzl_library(
    name = "toolchain_bzl",
    srcs = ["toolchain.bzl"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:toolchain_bzl",
        "//rs_bindings_from_cc/bazel_support:toolchain_bzl",
    ],
)

alias(
    name = "rust_api_from_cpp_toolchain_type",
    actual = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)

alias(
    name = "cpp_api_from_rust_toolchain_type",
    actual = "//cc_bindings_from_rs/bazel_support:toolchain_type",
)

alias(
    name = "rust_api_from_cpp_use_actual_bindings_generator",
    actual = "//rs_bindings_from_cc/bazel_support:use_actual_bindings_generator",
)

alias(
    name = "rust_api_from_cpp_use_actual_deps",
    actual = "//rs_bindings_from_cc/bazel_support:use_actual_deps",
)

alias(
    name = "rust_api_from_cpp_lazy_import_alien_decls",
    actual = "//rs_bindings_from_cc/bazel_support:lazy_import_alien_decls",
)

alias(
    name = "rust_api_from_cpp_generate_error_report",
    actual = "//rs_bindings_from_cc/bazel_support:generate_error_report",
)

alias(
    name = "cpp_api_from_rust_generate_error_report",
    actual = "//cc_bindings_from_rs/bazel_support:generate_error_report",
)

alias(
    name = "rust_api_from_cpp_legacy_mangling_allowlist",
    actual = "//rs_bindings_from_cc/bazel_support:legacy_mangling_allowlist",
)
