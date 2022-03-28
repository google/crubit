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
    "//third_party/bazel_rules/rules_rust/rust:defs.bzl",
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
    visibility = [
        "//devtools/rust/cc_interop:__subpackages__",
        "//third_party/crubit:__subpackages__",
    ],
)

deps_for_bindings(
    name = "deps_for_bindings",
    deps_for_generated_cc_file = [
        "//rs_bindings_from_cc/support:rs_api_impl_support",
    ],
    deps_for_generated_rs_file = [
        # Required for struct layout assertions added to the generated
        # Rust code.
        "//third_party/rust/memoffset/v0_6:memoffset_unstable_const",
        "//rs_bindings_from_cc/support:ctor",
        "//third_party/rust/pin_project/v1:pin_project",  # used by ctor macros
        # Required for `Copy` trait assertions added to the generated Rust
        # code.
        "//third_party/rust/static_assertions/v1:static_assertions",
    ],
    visibility = [
        "//devtools/rust/cc_interop:__subpackages__",
        "//third_party/crubit:__subpackages__",
    ],
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
        "//base",
        "//third_party/absl/container:flat_hash_map",
        "//third_party/absl/status",
        "//third_party/absl/status:statusor",
        "//third_party/absl/strings",
        "//rs_bindings_from_cc/util:status_macros",
        "@llvm///llvm:Support",
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_library(
    name = "bazel_types",
    hdrs = ["bazel_types.h"],
    deps = ["//rs_bindings_from_cc/util:string_type"],
)

cc_library(
    name = "cmdline",
    srcs = ["cmdline.cc"],
    hdrs = ["cmdline.h"],
    deps = [
        ":bazel_types",
        ":ir",
        "//third_party/absl/container:flat_hash_map",
        "//third_party/absl/flags:flag",
        "//third_party/absl/status:statusor",
        "//third_party/absl/strings",
        "//rs_bindings_from_cc/util:status_macros",
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
        "//rs_bindings_from_cc/util:check",
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
        ":ir",
        "//third_party/absl/container:flat_hash_map",
        "//third_party/absl/container:flat_hash_set",
        "//third_party/absl/status",
        "//third_party/absl/status:statusor",
        "//third_party/absl/strings",
        "//third_party/absl/strings:cord",
        "//third_party/absl/types:span",
        "//lifetime_annotations",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc/util:check",
        "//rs_bindings_from_cc/util:status_macros",
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
        "//third_party/absl/status",
        "//third_party/absl/status:statusor",
        "//third_party/absl/strings",
        "//third_party/absl/types:span",
        "@llvm///clang:ast",
    ],
)

cc_library(
    name = "ir",
    srcs = ["ir.cc"],
    hdrs = ["ir.h"],
    visibility = ["//devtools/rust/cc_interop:__subpackages__"],
    deps = [
        ":bazel_types",
        "//third_party/absl/strings",
        "//rs_bindings_from_cc/util:check",
        "//rs_bindings_from_cc/util:strong_int",
        "@llvm///llvm:Support",
    ],
)

rust_library(
    name = "rs_ir",
    srcs = ["ir.rs"],
    crate_name = "ir",
    deps = [
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/itertools/v0_9:itertools",
        "//third_party/rust/proc_macro2/v1:proc_macro2",
        "//third_party/rust/quote/v1:quote",
        "//third_party/rust/serde/v1:serde",
        "//third_party/rust/serde_json/v1:serde_json",
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
        ":rs_ffi_types",
        ":rs_ir",
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/itertools/v0_9:itertools",
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
        "//third_party/absl/container:flat_hash_map",
        "//third_party/absl/status",
        "//third_party/absl/status:statusor",
        "//third_party/absl/strings",
        "//third_party/absl/types:span",
        "//rs_bindings_from_cc/util:check",
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
        ":ffi_types",
        ":ir",
        ":ir_from_cc",
        "//third_party/absl/status:statusor",
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
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/itertools/v0_9:itertools",
        "//third_party/rust/quote/v1:quote",
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_library(
    name = "src_code_gen",
    srcs = ["src_code_gen.cc"],
    hdrs = ["src_code_gen.h"],
    deps = [
        ":ffi_types",
        ":ir",
        ":src_code_gen_impl",  # buildcleaner: keep
        "@llvm///clang:format",
        "@llvm///llvm:Support",
    ],
)

rust_library(
    name = "src_code_gen_impl",
    srcs = ["src_code_gen.rs"],
    deps = [
        ":rs_ffi_types",
        ":rs_ir",
        ":token_stream_printer",
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/itertools/v0_9:itertools",
        "//third_party/rust/proc_macro2/v1:proc_macro2",
        "//third_party/rust/quote/v1:quote",
        "//third_party/rust/serde_json/v1:serde_json",
        "//third_party/rust/syn/v1:syn",
    ],
)

rust_test(
    name = "src_code_gen_impl_test",
    crate = ":src_code_gen_impl",
    data = [
        "@rustfmt//:rustfmt.toml",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
    ],
    deps = [
        ":ir_testing",
        ":token_stream_matchers",
        "//third_party/rust/static_assertions/v1:static_assertions",
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

cc_library(
    name = "ffi_types",
    srcs = ["ffi_types.cc"],
    hdrs = ["ffi_types.h"],
    visibility = ["//fuzzer:__subpackages__"],
    deps = [
        ":rs_ffi_types",  # buildcleaner: keep
        "//third_party/absl/strings",
    ],
)

cc_library(
    name = "ast_convert",
    srcs = ["ast_convert.cc"],
    hdrs = ["ast_convert.h"],
    visibility = ["//devtools/rust/cc_interop:__subpackages__"],
    deps = [
        ":ir",
        "//third_party/absl/functional:function_ref",
        "//rs_bindings_from_cc/util:check",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
    ],
)

rust_library(
    name = "rs_ffi_types",
    srcs = ["ffi_types.rs"],
    crate_name = "ffi_types",
    visibility = ["//fuzzer:__subpackages__"],
)

rust_test(
    name = "rs_ffi_types_test",
    crate = ":rs_ffi_types",
)

rust_library(
    name = "token_stream_matchers",
    srcs = ["token_stream_matchers.rs"],
    deps = [
        ":rs_ir",
        ":token_stream_printer",
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/itertools/v0_9:itertools",
        "//third_party/rust/proc_macro2/v1:proc_macro2",
        "//third_party/rust/quote/v1:quote",
    ],
)

rust_test(
    name = "token_stream_matchers_test",
    crate = ":token_stream_matchers",
    data = [
        "@rustfmt//:rustfmt.toml",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
    ],
    deps = [
        ":ir_testing",
        "//third_party/unsupported_toolchains/rust/support:rust_okay_here",
    ],
)

rust_library(
    name = "token_stream_printer",
    srcs = ["token_stream_printer.rs"],
    data = [
        "@rustfmt//:rustfmt.toml",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
    ],
    deps = [
        "//third_party/rust/anyhow/v1:anyhow",
        "//third_party/rust/proc_macro2/v1:proc_macro2",
    ],
)

rust_test(
    name = "token_stream_printer_test",
    crate = ":token_stream_printer",
    data = [
        "@rustfmt//:rustfmt.toml",
        "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
    ],
    deps = [
        "//third_party/rust/quote/v1:quote",
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
    visibility = [
        "//devtools/rust/cc_interop:__subpackages__",
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
