"""Generates Rust bindings from C++ headers."""

load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "deps_for_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:with_cc_toolchain_flags.bzl",
    "with_cc_toolchain_flags",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_binary.bzl",
    "rust_bindings_from_cc_binary",
)
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:toolchain_headers.bzl",
    "bindings_for_toolchain_headers",
)

licenses(["notice"])

with_cc_toolchain_flags(
    name = "test_wrapper",
    binary = ":rs_bindings_from_cc",
    extra_args = [
        "--rs_out=/tmp/rs_api.rs",
        "--cc_out=/tmp/rs_api_impl.cc",
        "--ir_out=/tmp/ir.json",
    ],
)

rust_bindings_from_cc_binary(
    name = "rs_bindings_from_cc",
    binary = ":rs_bindings_from_cc_impl",
    visibility = ["//:__subpackages__"],
)

deps_for_bindings(
    name = "deps_for_bindings",
    deps_for_generated_cc_file = [
        "//rs_bindings_from_cc/support:rs_api_impl_support",
    ],
    deps_for_generated_rs_file = [
        # Required for struct layout assertions added to the generated
        # Rust code.
        "@crate_index//:memoffset",
        "//rs_bindings_from_cc/support:ctor",
        "//rs_bindings_from_cc/support:forward_declare",
        "//rs_bindings_from_cc/support:oops",
        # Required for `Copy` trait assertions added to the generated Rust
        # code.
        "@crate_index//:static_assertions",
    ],
    visibility = ["//:__subpackages__"],
)

cc_binary(
    name = "rs_bindings_from_cc_impl",
    srcs = ["rs_bindings_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline",
        ":generate_bindings_and_metadata",
        "//common:file_io",
        "//common:rust_allocator_shims",
        "//common:status_macros",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/flags:parse",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "generate_bindings_and_metadata",
    srcs = ["generate_bindings_and_metadata.cc"],
    hdrs = ["generate_bindings_and_metadata.h"],
    deps = [
        ":cc_collect_instantiations",
        ":cc_ir",
        ":cmdline",
        ":ir_from_cc",
        ":src_code_gen",
        "//common:status_macros",
        "@absl//absl/status:statusor",
    ],
)

cc_test(
    name = "generate_bindings_and_metadata_test",
    srcs = ["generate_bindings_and_metadata_test.cc"],
    deps = [
        ":cmdline",
        ":generate_bindings_and_metadata",
        "//common:rust_allocator_shims",
        "//common:test_utils",
        "@absl//absl/strings",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "ast_util",
    srcs = ["ast_util.cc"],
    hdrs = ["ast_util.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        "//common:check",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "bazel_types",
    srcs = ["bazel_types.cc"],
    hdrs = ["bazel_types.h"],
    deps = [
        "//common:check",
        "//common:string_type",
        "@absl//absl/strings",
    ],
)

cc_test(
    name = "bazel_types_test",
    srcs = ["bazel_types_test.cc"],
    deps = [
        ":bazel_types",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "cmdline",
    srcs = ["cmdline.cc"],
    hdrs = ["cmdline.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        "//common:status_macros",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/flags:flag",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "cmdline_test",
    srcs = ["cmdline_test.cc"],
    deps = [
        ":bazel_types",
        ":cmdline",
        "//common:status_test_matchers",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "decl_importer",
    hdrs = ["decl_importer.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        "cc_ir",
        ":bazel_types",
        "//lifetime_annotations",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/status:statusor",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "frontend_action",
    srcs = ["frontend_action.cc"],
    hdrs = ["frontend_action.h"],
    deps = [
        ":ast_consumer",
        ":decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "ast_consumer",
    srcs = ["ast_consumer.cc"],
    hdrs = ["ast_consumer.h"],
    deps = [
        ":decl_importer",
        ":importer",
        "//common:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "importer",
    srcs = ["importer.cc"],
    hdrs = ["importer.h"],
    deps = [
        ":ast_convert",
        ":bazel_types",
        ":cc_ir",
        ":decl_importer",
        "//common:check",
        "//common:status_macros",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc/importers:class_template",
        "//rs_bindings_from_cc/importers:cxx_record",
        "//rs_bindings_from_cc/importers:enum",
        "//rs_bindings_from_cc/importers:function",
        "//rs_bindings_from_cc/importers:function_template",
        "//rs_bindings_from_cc/importers:namespace",
        "//rs_bindings_from_cc/importers:typedef_name",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/strings:cord",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "importer_test",
    srcs = ["importer_test.cc"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":ir_from_cc",
        "//common:status_test_matchers",
        "//common:test_utils",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "cc_ir",
    srcs = ["ir.cc"],
    hdrs = ["ir.h"],
    deps = [
        ":bazel_types",
        "//common:check",
        "//common:strong_int",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "ir",
    srcs = ["ir.rs"],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "rs_ir_test",
    crate = ":ir",
    deps = [
        ":token_stream_matchers",
    ],
)

rust_library(
    name = "ir_testing",
    testonly = 1,
    srcs = ["ir_testing.rs"],
    deps = [
        ":ir",
        ":json_from_cc",
        "//common:ffi_types",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
    ],
)

cc_library(
    name = "ir_from_cc",
    srcs = ["ir_from_cc.cc"],
    hdrs = ["ir_from_cc.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":frontend_action",
        ":importer",
        "//common:check",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
    ],
)

cc_library(
    name = "json_from_cc",
    srcs = ["json_from_cc.cc"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":ir_from_cc",
        "//common:cc_ffi_types",
        "@absl//absl/status:statusor",
        "@llvm-project//llvm:Support",
    ],
)

rust_test(
    name = "ir_from_cc_test",
    srcs = ["ir_from_cc_test.rs"],
    deps = [
        ":ir",
        ":ir_testing",
        ":token_stream_matchers",
        "//common:rust_allocator_shims",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

cc_library(
    name = "src_code_gen",
    srcs = ["src_code_gen.cc"],
    hdrs = ["src_code_gen.h"],
    deps = [
        ":cc_ir",
        ":src_code_gen_impl",  # buildcleaner: keep
        "//common:cc_ffi_types",
        "//common:status_macros",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:format",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "src_code_gen_impl",
    srcs = ["src_code_gen.rs"],
    deps = [
        ":ir",
        "//common:ffi_types",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "src_code_gen_impl_test",
    crate = ":src_code_gen_impl",
    data = [
    ],
    deps = [
        ":ir_testing",
        ":token_stream_matchers",
        "//common:rust_allocator_shims",
        "@crate_index//:static_assertions",
    ],
)

cc_library(
    name = "ast_convert",
    srcs = ["ast_convert.cc"],
    hdrs = ["ast_convert.h"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        ":cc_ir",
        "//common:check",
        "@absl//absl/functional:function_ref",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

rust_library(
    name = "token_stream_matchers",
    srcs = ["token_stream_matchers.rs"],
    deps = [
        ":ir",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

rust_test(
    name = "token_stream_matchers_test",
    crate = ":token_stream_matchers",
    data = [
    ],
    deps = [
        ":ir_testing",
        "//common:rust_allocator_shims",
    ],
)

LIBCXX_HEADERS = [
    "cstddef",
    "cstdlib",
    "version",
    "limits",
    "climits",
    "cfloat",
    "cstdint",
    "new",
    "typeinfo",
    "exception",
    "initializer_list",
    "compare",
    "csignal",
    "csetjmp",
    "cstdarg",
    "string",
    "memory",
    "stdint.h",
    "stddef.h",
    "stdlib.h",
]

LIBC_HEADERS = []

config_setting(
    name = "llvm_unstable",
    values = {
        "crosstool_top": "//nowhere/llvm:everything",
    },
)

filegroup(
    name = "toolchain_headers",
    srcs = select({
        ":llvm_unstable": ["//nowhere/llvm:cc-std-headers"],
        "//conditions:default": ["//nowhere/llvm:cc-std-headers"],
    }) + [
        "//third_party/grte/v5_x86/release/usr/grte/v5:compile",
    ],
    visibility = ["//visibility:private"],
)

filegroup(
    name = "builtin_headers",
    srcs = [
        "@llvm-project//clang:builtin_headers_filegroup",
        "@llvm-project//compiler-rt:fuzzer_filegroup",
        "@llvm-project//compiler-rt:sanitizer_filegroup",
    ],
    visibility = ["//visibility:public"],
)

bindings_for_toolchain_headers(
    name = "cc_std",
    hdrs = ":toolchain_headers",
    public_libc_hdrs = LIBC_HEADERS,
    public_libcxx_hdrs = LIBCXX_HEADERS,
    visibility = ["//visibility:public"],
)

cc_library(
    name = "cc_collect_instantiations",
    srcs = ["collect_instantiations.cc"],
    hdrs = ["collect_instantiations.h"],
    deps = [
        ":collect_instantiations",
        "//common:cc_ffi_types",
        "//common:check",
        "@absl//absl/status:statusor",
        "@absl//absl/types:span",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "cc_collect_instantiations_test",
    srcs = ["collect_instantiations_test.cc"],
    deps = [
        ":cc_collect_instantiations",
        "//common:rust_allocator_shims",
        "//common:status_test_matchers",
        "//common:test_utils",
        "@com_google_googletest//:gtest_main",
    ],
)

rust_library(
    name = "collect_instantiations",
    srcs = ["collect_instantiations.rs"],
    deps = [
        "//common:ffi_types",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "collect_instantiations_test",
    args = [
        # To prevent tests from polluting the environment variables for each other.
        "--test-threads",
        "1",
    ],
    crate = ":collect_instantiations",
    deps = [
        "@crate_index//:quote",
    ],
)
