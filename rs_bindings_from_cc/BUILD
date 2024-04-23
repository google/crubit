"""Generates Rust bindings from C++ headers."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:multiplatform_testing.bzl",
    "multiplatform_rust_test",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_binary",
    "crubit_cc_test",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "deps_for_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_binary.bzl",
    "rust_bindings_from_cc_binary",
)
load(
    "//rs_bindings_from_cc/bazel_support:with_cc_toolchain_flags.bzl",
    "with_cc_toolchain_flags",
)

package(default_applicable_licenses = ["//:license"])

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
        "//support/internal:bindings_support",
    ],
    deps_for_generated_rs_file = [
        "//support:ctor",
        "//support:forward_declare",
        "//support:oops",
        # Required for `Copy` trait assertions added to the generated Rust
        # code.
        "@crate_index//:static_assertions",
    ],
    visibility = ["//:__subpackages__"],
)

crubit_cc_binary(
    name = "rs_bindings_from_cc_impl",
    srcs = ["rs_bindings_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        "//common:file_io",
        "//common:status_macros",
        "@abseil-cpp//absl/flags:parse",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
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
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
    ],
)

crubit_cc_test(
    name = "generate_bindings_and_metadata_test",
    srcs = ["generate_bindings_and_metadata_test.cc"],
    data = [
    ],
    tags = [
        # This test uses e.g. rustfmt and is difficult to run on ARM. We could run it using
        # cross-compilation, but it's mostly uninteresting from a multi-platform POV.
        "not_run:arm",
    ],
    deps = [
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        "//common:cc_ffi_types",
        "//common:status_macros",
        "//common:test_utils",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "ast_util",
    srcs = ["ast_util.cc"],
    hdrs = ["ast_util.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        "@abseil-cpp//absl/functional:function_ref",
        "@abseil-cpp//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "bazel_types",
    srcs = [],
    hdrs = ["bazel_types.h"],
    visibility = [
        ":__subpackages__",
    ],
    deps = [
        "//common:string_type",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
    ],
)

cc_library(
    name = "type_map",
    srcs = ["type_map.cc"],
    hdrs = ["type_map.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":cc_ir",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/strings",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "cmdline",
    srcs = ["cmdline.cc"],
    hdrs = ["cmdline.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline_flags",
        "//common:cc_ffi_types",
        "//common:status_macros",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/debugging:leak_check",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "cmdline_flags",
    hdrs = ["cmdline_flags.h"],
    visibility = ["//visibility:private"],
    deps = ["@abseil-cpp//absl/flags:flag"],
)

crubit_cc_test(
    name = "cmdline_test",
    srcs = ["cmdline_test.cc"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline",
        ":cmdline_flags",
        "//common:cc_ffi_types",
        "//common:status_macros",
        "//common:status_test_matchers",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
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
        "//lifetime_annotations:type_lifetimes",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
    ],
)

cc_library(
    name = "frontend_action",
    srcs = ["frontend_action.cc"],
    hdrs = ["frontend_action.h"],
    deps = [
        ":ast_consumer",
        ":decl_importer",
        "//lifetime_annotations",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "ast_consumer",
    srcs = ["ast_consumer.cc"],
    hdrs = ["ast_consumer.h"],
    deps = [
        ":decl_importer",
        ":importer",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
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
        ":type_map",
        "//common:status_macros",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:recording_diagnostic_consumer",
        "//rs_bindings_from_cc/importers:class_template",
        "//rs_bindings_from_cc/importers:cxx_record",
        "//rs_bindings_from_cc/importers:enum",
        "//rs_bindings_from_cc/importers:friend",
        "//rs_bindings_from_cc/importers:function",
        "//rs_bindings_from_cc/importers:function_template",
        "//rs_bindings_from_cc/importers:namespace",
        "//rs_bindings_from_cc/importers:type_alias",
        "//rs_bindings_from_cc/importers:type_map_override",
        "@abseil-cpp//absl/base:no_destructor",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/log:die_if_null",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:cord",
        "@abseil-cpp//absl/strings:str_format",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "importer_test",
    srcs = ["importer_test.cc"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":ir_from_cc",
        "//common:status_test_matchers",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/strings",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "cc_ir",
    srcs = ["ir.cc"],
    hdrs = ["ir.h"],
    visibility = ["//rs_bindings_from_cc:__subpackages__"],
    deps = [
        ":bazel_types",
        "//common:string_type",
        "//common:strong_int",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:str_format",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "ir",
    srcs = ["ir.rs"],
    visibility = ["//rs_bindings_from_cc/generate_bindings:__subpackages__"],
    deps = [
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

crubit_rust_test(
    name = "rs_ir_test",
    crate = ":ir",
)

rust_library(
    name = "ir_testing",
    testonly = 1,
    srcs = ["ir_testing.rs"],
    visibility = ["//rs_bindings_from_cc/generate_bindings:__subpackages__"],
    deps = [
        ":ir",
        ":json_from_cc",
        "//common:arc_anyhow",
        "//common:ffi_types",
        "//common:multiplatform_testing",
        "@crate_index//:flagset",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
    ],
)

crubit_rust_test(
    name = "ir_testing_test",
    crate = ":ir_testing",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
    ],
    deps = [
        ":ir_matchers",
        ":ir_testing",
        "@crate_index//:quote",
    ],
)

cc_library(
    name = "ir_from_cc",
    srcs = ["ir_from_cc.cc"],
    hdrs = ["ir_from_cc.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":decl_importer",
        ":frontend_action",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:serialization",
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
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

multiplatform_rust_test(
    name = "ir_from_cc_test",
    srcs = ["ir_from_cc_test.rs"],
    tags = ["not_run:arm"],
    deps = [
        ":ir",
        ":ir_matchers",
        ":ir_testing",
        "//common:arc_anyhow",
        "//common:multiplatform_testing",
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
        "//common:cc_ffi_types",
        "//common:status_macros",
        "//rs_bindings_from_cc/generate_bindings",  # buildcleaner: keep
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "error_report",
    srcs = ["error_report.rs"],
    visibility = ["//rs_bindings_from_cc/generate_bindings:__subpackages__"],
    deps = [
        "//common:arc_anyhow",
        "@crate_index//:anyhow",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

crubit_rust_test(
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
        "@abseil-cpp//absl/functional:function_ref",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "recording_diagnostic_consumer",
    srcs = ["recording_diagnostic_consumer.cc"],
    hdrs = ["recording_diagnostic_consumer.h"],
    visibility = [
        "//rs_bindings_from_cc:__subpackages__",
    ],
    deps = [
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:str_format",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
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

crubit_rust_test(
    name = "ir_matchers_test",
    crate = ":ir_matchers",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:multiplatform_testing",
    ],
)

filegroup(
    name = "builtin_headers",
    srcs = [
        "//llvm/llvm-project/clang:builtin_headers_filegroup",
        "//llvm/llvm-project/compiler-rt:fuzzer_filegroup",
        "//llvm/llvm-project/compiler-rt:sanitizer_filegroup",
    ],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "cc_collect_instantiations",
    srcs = ["collect_instantiations.cc"],
    hdrs = ["collect_instantiations.h"],
    deps = [
        ":collect_instantiations",  # build_cleaner: keep
        "//common:cc_ffi_types",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "cc_collect_instantiations_test",
    srcs = ["collect_instantiations_test.cc"],
    deps = [
        ":cc_collect_instantiations",
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

crubit_rust_test(
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
        "@abseil-cpp//absl/container:btree",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "collect_namespaces_test",
    srcs = ["collect_namespaces_test.cc"],
    deps = [
        ":cc_ir",
        ":collect_namespaces",
        ":ir_from_cc",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
        "@com_google_googletest//:gtest_main",
    ],
)
