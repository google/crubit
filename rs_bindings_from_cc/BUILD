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
        "//support/internal:rs_api_impl_support",
    ],
    deps_for_generated_rs_file = [
        # Required for struct layout assertions added to the generated
        # Rust code.
        "@crate_index//:memoffset",
        "//support:ctor",
        "//support:forward_declare",
        "//support:oops",
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
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        "//common:file_io",
        "//common:rust_allocator_shims",
        "//common:status_macros",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/flags:parse",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:format",
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
        ":collect_namespaces",
        ":ir_from_cc",
        ":src_code_gen",
        "//common:status_macros",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
    ],
)

cc_test(
    name = "generate_bindings_and_metadata_test",
    srcs = ["generate_bindings_and_metadata_test.cc"],
    data = [
    ],
    deps = [
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        "//common:rust_allocator_shims",
        "//common:status_macros",
        "//common:test_utils",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
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
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "bazel_types",
    srcs = ["bazel_types.cc"],
    hdrs = ["bazel_types.h"],
    deps = [
        "//common:string_type",
        "@absl//absl/log:check",
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
    name = "known_types_map",
    srcs = ["known_types_map.cc"],
    hdrs = ["known_types_map.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/strings",
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
        "@absl//absl/log",
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
        "@absl//absl/log:check",
        "@absl//absl/status:statusor",
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
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "importer",
    srcs = ["importer.cc"],
    hdrs = ["importer.h"],
    deps = [
        ":ast_util",
        ":bazel_types",
        ":cc_ir",
        ":decl_importer",
        ":known_types_map",
        "//common:status_macros",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc/importers:class_template",
        "//rs_bindings_from_cc/importers:cxx_record",
        "//rs_bindings_from_cc/importers:enum",
        "//rs_bindings_from_cc/importers:friend",
        "//rs_bindings_from_cc/importers:function",
        "//rs_bindings_from_cc/importers:function_template",
        "//rs_bindings_from_cc/importers:namespace",
        "//rs_bindings_from_cc/importers:typedef_name",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/log",
        "@absl//absl/log:check",
        "@absl//absl/log:die_if_null",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/strings:cord",
        "@absl//absl/strings:str_format",
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
        "//common:strong_int",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/log:check",
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
        "//common:arc_anyhow",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

rust_test(
    name = "rs_ir_test",
    crate = ":ir",
)

rust_library(
    name = "ir_testing",
    testonly = 1,
    srcs = ["ir_testing.rs"],
    deps = [
        ":ir",
        ":json_from_cc",
        "//common:arc_anyhow",
        "//common:ffi_types",
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
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/log:check",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
    ],
)

cc_library(
    name = "json_from_cc",
    testonly = 1,
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
        ":ir_matchers",
        ":ir_testing",
        "//common:arc_anyhow",
        "//common:rust_allocator_shims",
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
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "src_code_gen_impl",
    srcs = ["src_code_gen.rs"],
    deps = [
        ":error_report",
        ":ir",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:ffi_types",
        "//common:token_stream_printer",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:salsa",
        "@crate_index//:serde_json",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "src_code_gen_impl_test",
    crate = ":src_code_gen_impl",
    deps = [
        ":ir_matchers",
        ":ir_testing",
        "//common:rust_allocator_shims",
        "//common:token_stream_matchers",
        "@crate_index//:static_assertions",
    ],
)

rust_library(
    name = "error_report",
    srcs = ["error_report.rs"],
    deps = [
        "//common:arc_anyhow",
        "@crate_index//:anyhow",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

rust_test(
    name = "error_report_test",
    crate = ":error_report",
    deps = [
        "@crate_index//:serde_json",
    ],
)

cc_library(
    name = "ast_convert",
    srcs = ["ast_convert.cc"],
    hdrs = ["ast_convert.h"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        ":cc_ir",
        "@absl//absl/functional:function_ref",
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

rust_library(
    name = "ir_matchers",
    testonly = 1,
    srcs = ["ir_matchers.rs"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":ir",
        ":ir_testing",
        "//common:token_stream_matchers",
        "//common:token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

rust_test(
    name = "ir_matchers_test",
    crate = ":ir_matchers",
    deps = [
        "//common:rust_allocator_shims",
    ],
)

LIBCXX_HEADERS = [
    "algorithm",
    "any",
    "array",
    "atomic",
    "bitset",
    "cassert",
    "cctype",
    "cerrno",
    "cfenv",
    "cfloat",
    "charconv",
    "chrono",
    "cinttypes",
    "climits",
    "cmath",
    "condition_variable",
    "csetjmp",
    "csignal",
    "cstdarg",
    "cstddef",
    "cstdint",
    "cstdio",
    "cstdlib",
    "cstring",
    "ctime",
    "cuchar",
    "cwchar",
    "cwctype",
    "deque",
    "exception",
    "forward_list",
    "functional",
    "initializer_list",
    "ios",
    "limits",
    "list",
    "map",
    "memory",
    "mutex",
    "new",
    "numeric",
    "optional",
    "queue",
    "ratio",
    "set",
    "scoped_allocator",
    "stack",
    "stdexcept",
    "streambuf",
    "string",
    "string_view",
    "system_error",
    "tuple",
    "type_traits",
    "typeindex",
    "typeinfo",
    "unordered_map",
    "unordered_set",
    "utility",
    "variant",
    # "vector", # TODO(b/248542210): include vector when we can conditionally import methods.
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
    extra_rs_srcs = ["//support/extra_rs_srcs_for_cc_std"],
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
        "//common:arc_anyhow",
        "//common:ffi_types",
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

cc_library(
    name = "collect_namespaces",
    srcs = ["collect_namespaces.cc"],
    hdrs = ["collect_namespaces.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        "@absl//absl/container:btree",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "collect_namespaces_test",
    srcs = ["collect_namespaces_test.cc"],
    deps = [
        ":cc_ir",
        ":collect_namespaces",
        ":ir_from_cc",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@com_google_googletest//:gtest_main",
    ],
)
