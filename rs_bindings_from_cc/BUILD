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
    visibility = ["//third_party/crubit:__subpackages__"],
)

deps_for_bindings(
    name = "deps_for_bindings",
    deps_for_generated_cc_file = [
        "//rs_bindings_from_cc/support:rs_api_impl_support",
    ],
    deps_for_generated_rs_file = [
        # Required for struct layout assertions added to the generated
        # Rust code.
        "@crates_io//memoffset/v0_6:memoffset_unstable_const",
        "//rs_bindings_from_cc/support:ctor",
        "//rs_bindings_from_cc/support:forward_declare",
        "@crates_io//pin_project/v1:pin_project",  # used by ctor macros
        # Required for `Copy` trait assertions added to the generated Rust
        # code.
        "@crates_io//static_assertions/v1:static_assertions",
    ],
    visibility = ["//third_party/crubit:__subpackages__"],
)

cc_binary(
    name = "rs_bindings_from_cc_impl",
    srcs = ["rs_bindings_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":bazel_types",
        ":cmdline",
        ":ir",
        ":ir_from_cc",
        ":src_code_gen",
        "//common:file_io",
        "//common:status_macros",
        "@absl//container:flat_hash_map",
        "@absl//flags:parse",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "@llvm///llvm:Support",
        "@rust//support:rust_okay_here",
    ],
)

cc_library(
    name = "bazel_types",
    hdrs = ["bazel_types.h"],
    deps = ["//common:string_type"],
)

cc_library(
    name = "cmdline",
    srcs = ["cmdline.cc"],
    hdrs = ["cmdline.h"],
    deps = [
        ":bazel_types",
        ":ir",
        "//common:status_macros",
        "@absl//container:flat_hash_map",
        "@absl//flags:flag",
        "@absl//status:statusor",
        "@absl//strings",
        "@llvm///llvm:Support",
    ],
)

cc_test(
    name = "cmdline_test",
    srcs = ["cmdline_test.cc"],
    deps = [
        ":bazel_types",
        ":cmdline",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "decl_importer",
    hdrs = ["decl_importer.h"],
    visibility = ["//third_party/crubit:__subpackages__"],
    deps = [
        "ir",
        ":bazel_types",
        "//lifetime_annotations",
        "@absl//container:flat_hash_map",
        "@absl//status:statusor",
    ],
)

cc_library(
    name = "frontend_action",
    srcs = ["frontend_action.cc"],
    hdrs = ["frontend_action.h"],
    deps = [
        ":ast_consumer",
        ":importer",
        "//lifetime_annotations",
        "@llvm///clang:ast",
        "@llvm///clang:frontend",
    ],
)

cc_library(
    name = "ast_consumer",
    srcs = ["ast_consumer.cc"],
    hdrs = ["ast_consumer.h"],
    deps = [
        ":importer",
        "//common:check",
        "@llvm///clang:ast",
        "@llvm///clang:frontend",
    ],
)

cc_library(
    name = "importer",
    srcs = ["importer.cc"],
    hdrs = ["importer.h"],
    deps = [
        ":ast_convert",
        ":bazel_types",
        ":decl_importer",
        ":ir",
        "//common:check",
        "//common:status_macros",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc/importers:class_template",
        "//rs_bindings_from_cc/importers:cxx_record",
        "//rs_bindings_from_cc/importers:enum",
        "//rs_bindings_from_cc/importers:function",
        "//rs_bindings_from_cc/importers:function_template",
        "//rs_bindings_from_cc/importers:namespace",
        "//rs_bindings_from_cc/importers:typedef_name",
        "@absl//container:flat_hash_map",
        "@absl//container:flat_hash_set",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "@absl//strings:cord",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
        "@llvm///clang:sema",
        "@llvm///llvm:Support",
    ],
)

cc_test(
    name = "importer_test",
    srcs = ["importer_test.cc"],
    deps = [
        ":bazel_types",
        ":ir",
        ":ir_from_cc",
        "//testing/base/public:gunit_main",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "@absl//types:span",
        "@llvm///clang:ast",
    ],
)

cc_library(
    name = "ir",
    srcs = ["ir.cc"],
    hdrs = ["ir.h"],
    deps = [
        ":bazel_types",
        "//common:check",
        "//common:strong_int",
        "@absl//strings",
        "@llvm///clang:ast",
        "@llvm///llvm:Support",
    ],
)

rust_library(
    name = "rs_ir",
    srcs = ["ir.rs"],
    crate_name = "ir",
    deps = [
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//itertools/v0_10:itertools",
        "@crates_io//proc_macro2/v1:proc_macro2",
        "@crates_io//quote/v1:quote",
        "@crates_io//serde/v1:serde",
        "@crates_io//serde_json/v1:serde_json",
    ],
)

rust_test(
    name = "rs_ir_test",
    crate = ":rs_ir",
)

rust_library(
    name = "ir_testing",
    testonly = 1,
    srcs = ["ir_testing.rs"],
    deps = [
        ":json_from_cc",
        ":rs_ir",
        "//common:rs_ffi_types",
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//itertools/v0_10:itertools",
    ],
)

cc_library(
    name = "ir_from_cc",
    srcs = ["ir_from_cc.cc"],
    hdrs = ["ir_from_cc.h"],
    deps = [
        ":bazel_types",
        ":frontend_action",
        ":importer",
        ":ir",
        "//common:check",
        "@absl//container:flat_hash_map",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "@absl//types:span",
        "@llvm///clang:basic",
        "@llvm///clang:frontend",
        "@llvm///clang:tooling",
    ],
)

cc_library(
    name = "json_from_cc",
    srcs = ["json_from_cc.cc"],
    deps = [
        ":bazel_types",
        ":ir",
        ":ir_from_cc",
        "//common:ffi_types",
        "@absl//status:statusor",
        "@llvm///llvm:Support",
    ],
)

rust_test(
    name = "ir_from_cc_test",
    srcs = ["ir_from_cc_test.rs"],
    deps = [
        ":ir_testing",
        ":rs_ir",
        ":token_stream_matchers",
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//itertools/v0_10:itertools",
        "@crates_io//quote/v1:quote",
        "@rust//support:rust_okay_here",
    ],
)

cc_library(
    name = "src_code_gen",
    srcs = ["src_code_gen.cc"],
    hdrs = ["src_code_gen.h"],
    deps = [
        ":ir",
        ":src_code_gen_impl",  # buildcleaner: keep
        "//common:ffi_types",
        "@llvm///clang:format",
        "@llvm///llvm:Support",
    ],
)

rust_library(
    name = "src_code_gen_impl",
    srcs = ["src_code_gen.rs"],
    deps = [
        ":rs_ir",
        "//common:rs_ffi_types",
        "//common:token_stream_printer",
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//itertools/v0_10:itertools",
        "@crates_io//proc_macro2/v1:proc_macro2",
        "@crates_io//quote/v1:quote",
        "@crates_io//serde_json/v1:serde_json",
        "@crates_io//syn/v1:syn",
    ],
)

rust_test(
    name = "src_code_gen_impl_test",
    crate = ":src_code_gen_impl",
    data = [
        "@rust//toolchains/nightly:bin/rustfmt",
        "@rustfmt//:rustfmt.toml",
    ],
    deps = [
        ":ir_testing",
        ":token_stream_matchers",
        "@crates_io//static_assertions/v1:static_assertions",
        "@rust//support:rust_okay_here",
    ],
)

cc_library(
    name = "ast_convert",
    srcs = ["ast_convert.cc"],
    hdrs = ["ast_convert.h"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        ":ir",
        "//common:check",
        "@absl//functional:function_ref",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
    ],
)

rust_library(
    name = "token_stream_matchers",
    srcs = ["token_stream_matchers.rs"],
    deps = [
        ":rs_ir",
        "//common:token_stream_printer",
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//itertools/v0_10:itertools",
        "@crates_io//proc_macro2/v1:proc_macro2",
        "@crates_io//quote/v1:quote",
    ],
)

rust_test(
    name = "token_stream_matchers_test",
    crate = ":token_stream_matchers",
    data = [
        "@rust//toolchains/nightly:bin/rustfmt",
        "@rustfmt//:rustfmt.toml",
    ],
    deps = [
        ":ir_testing",
        "@rust//support:rust_okay_here",
    ],
)

STD_HEADERS = [
    "cstddef",
    "cstdlib",
    "version",
    "limits",
    "climits",
    "cfloat",
    "cstdint",
    "new",
    "typeinfo",
    "source_location",
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

config_setting(
    name = "llvm_unstable",
    values = {
        "crosstool_top": "@clang//:everything",
    },
)

filegroup(
    name = "toolchain_headers",
    srcs = select({
        ":llvm_unstable": ["@clang//:cc-std-headers"],
        "//conditions:default": ["@clang//:cc-std-headers"],
    }) + [
        "//third_party/grte/v5_x86/release/usr/grte/v5:compile",
    ],
)

filegroup(
    name = "builtin_headers",
    srcs = [
        "@llvm///clang:builtin_headers_filegroup",
        "@llvm///compiler-rt:fuzzer_filegroup",
        "@llvm///compiler-rt:sanitizer_filegroup",
    ],
    visibility = ["//visibility:public"],
)

bindings_for_toolchain_headers(
    name = "cc_std",
    hdrs = ":toolchain_headers",
    public_hdrs = STD_HEADERS,
    visibility = ["//visibility:public"],
)
