"""Generates Rust bindings from C++ headers."""

load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@protobuf//rust:defs.bzl", "rust_proto_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//common:multiplatform_testing.bzl",
    "multiplatform_rust_test",
)
load("//common/bazel_support:external_binaries.bzl", "EXTERNAL_BINARIES")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
    "crubit_rust_binary",
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

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

config_setting(
    name = "enable_heap_profiling",
    define_values = {"enable_heap_profiling": "1"},
)

_PROFILING_RUSTC_FLAGS = select({
    ":enable_heap_profiling": ["--cfg=enable_heap_profiling"],
    "//conditions:default": [],
})

rust_bindings_from_cc_binary(
    name = "rs_bindings_from_cc",
    binary = ":rs_bindings_from_cc_main",
    visibility = ["//:__subpackages__"],
)

deps_for_bindings(
    name = "deps_for_bindings",
    deps_for_generated_cc_file = [
        "//support/internal:bindings_support",
        "//support:bridge_cpp",
        "//support/rs_std:dyn_callable",
        "//support/rs_std:lossy_formatter_for_bindings",
    ],
    deps_for_generated_rs_file = [
        "//support:ctor",
        "//support:operator",
        "//support/ffi_11",
        "//support:forward_declare",
        "//support:oops",
        "//support:bridge_rust",
        "//support:cref",
        "//support/rs_std:dyn_callable_rs",
        "//support/rs_std:lossy_formatter",
        "//support:crubit_annotate",
        "//support/extract_cpp_from_rust:inline_cpp_macro",
        # Required for `Copy` trait assertions added to the generated Rust
        # code.
        "@crate_index//:static_assertions",  # v1
        # Required for `cxx.rs` integration.
        "@crate_index//:cxx",  # v1
    ],
    visibility = ["//:__subpackages__"],
)

crubit_rust_binary(
    name = "rs_bindings_from_cc_main",
    srcs = ["rs_bindings_from_cc.rs"],
    # Do not add a fake dependency on Crubit, because it will introduce a
    # dependency cycle.
    crubit_dep = False,
    visibility = ["//visibility:public"],
    deps = [
        ":rs_bindings_from_cc_impl",
    ],
)

cc_library(
    name = "rs_bindings_from_cc_impl",
    srcs = ["rs_bindings_from_cc.cc"],
    # Defines CRUBIT_HEAP_PROFILING when --define=enable_heap_profiling=1 is passed,
    # enabling heap profile recording in rs_bindings_from_cc.cc.
    defines = select({
        ":enable_heap_profiling": ["CRUBIT_HEAP_PROFILING"],
        "//conditions:default": [],
    }),
    deps = [
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        ":profiling",
        "//common:file_io",
        "//common:status_macros",
        "//third_party/protobuf",
        "@abseil-cpp//absl/flags:parse",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//llvm:Support",
    ],
    alwayslink = 1,
)

cc_library(
    name = "generate_bindings_and_metadata",
    srcs = ["generate_bindings_and_metadata.cc"],
    hdrs = ["generate_bindings_and_metadata.h"],
    visibility = [
        "//cargo:__subpackages__",
    ],
    deps = [
        ":cc_collect_instantiations",
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":ir_from_cc",
        ":profiling",
        ":src_code_gen",
        "//common:status_macros",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "generate_bindings_and_metadata_test",
    srcs = ["generate_bindings_and_metadata_test.cc"],
    data = [
    ],
    local_defines = EXTERNAL_BINARIES,
    tags = [
        # This test uses e.g. rustfmt and is difficult to run cross-platform. We could run it using
        # cross-compilation, but it's mostly uninteresting from a multi-platform POV.
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":cc_ir",
        ":cmdline",
        ":collect_namespaces",
        ":generate_bindings_and_metadata",
        "//common:cc_ffi_types",
        "//common:external_binaries_cc",
        "//common:status_macros",
        "//common:test_utils",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@googletest//:gtest_main",
    ],
)

cc_library(
    name = "ast_util",
    srcs = ["ast_util.cc"],
    hdrs = ["ast_util.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":cc_ir",
        ":decl_importer",
        ":recording_diagnostic_consumer",
        "//common:annotation_reader",
        "//common:status_macros",
        "//common:string_view_conversion",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/functional:function_ref",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "bazel_types",
    srcs = [],
    hdrs = ["bazel_types.h"],
    deps = ["//common:string_type"],
)

cc_library(
    name = "profiling",
    srcs = ["profiling.cc"],
    hdrs = ["profiling.h"],
    deps = [
        "@abseil-cpp//absl/strings:string_view",
    ] + select({
        ":enable_heap_profiling": [
            "//file/localfile",
            "//perftools/profiles/collector/heap:alloc_recorder",
        ],
        "//conditions:default": [],
    }),
)

cc_library(
    name = "type_map",
    srcs = ["type_map.cc"],
    hdrs = ["type_map.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":cc_ir",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "annotations_consumer",
    srcs = ["annotations_consumer.cc"],
    hdrs = ["annotations_consumer.h"],
    deps = [
        ":cc_ir",
        "//common:annotation_reader",
        "@abseil-cpp//absl/base:no_destructor",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "cmdline",
    srcs = ["cmdline.cc"],
    hdrs = ["cmdline.h"],
    visibility = [
        "//cargo:__subpackages__",
    ],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline_flags",
        "//common:status_macros",
        "//common:string_view_conversion",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/debugging:leak_check",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "cmdline_flags",
    hdrs = ["cmdline_flags.h"],
    deps = [
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

crubit_cc_test(
    name = "cmdline_test",
    srcs = ["cmdline_test.cc"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline",
        ":cmdline_flags",
        "//common:status_macros",
        "//common:status_test_matchers",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_library(
    name = "decl_importer",
    hdrs = ["decl_importer.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        "cc_ir",
        ":bazel_types",
        ":cmdline_flags",
        ":ir_cc_proto",
        "//lifetime_annotations",
        "//lifetime_annotations:type_lifetimes",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
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
    ],
)

cc_library(
    name = "importer",
    srcs = ["importer.cc"],
    hdrs = ["importer.h"],
    deps = [
        ":annotations_consumer",
        ":ast_util",
        ":bazel_types",
        ":cc_ir",
        ":cmdline_flags",
        ":decl_importer",
        ":ir_cc_proto",
        ":recording_diagnostic_consumer",
        ":type_map",
        "//common:annotation_reader",
        "//common:status_macros",
        "//common:string_view_conversion",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc/importers:class_template",
        "//rs_bindings_from_cc/importers:cxx_record",
        "//rs_bindings_from_cc/importers:enum",
        "//rs_bindings_from_cc/importers:enum_constant",
        "//rs_bindings_from_cc/importers:existing_rust_type",
        "//rs_bindings_from_cc/importers:friend",
        "//rs_bindings_from_cc/importers:function",
        "//rs_bindings_from_cc/importers:function_template",
        "//rs_bindings_from_cc/importers:namespace",
        "//rs_bindings_from_cc/importers:type_alias",
        "//rs_bindings_from_cc/importers:var",
        "@abseil-cpp//absl/algorithm:container",
        "@abseil-cpp//absl/base:no_destructor",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/log:die_if_null",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:lex",
        "@llvm-project//clang:sema",
        "@llvm-project//clang:unified_symbol_resolution",
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
        "//net/proto2/contrib/proto_matcher",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/functional:overload",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
    ],
)

proto_library(
    name = "ir_proto",
    srcs = ["ir.proto"],
    visibility = ["//:__subpackages__"],
)

cc_proto_library(
    name = "ir_cc_proto",
    visibility = ["//:__subpackages__"],
    deps = [":ir_proto"],
)

rust_proto_library(
    name = "ir_rust_proto",
    visibility = ["//:__subpackages__"],
    deps = [":ir_proto"],
)

cc_library(
    name = "cc_ir",
    srcs = ["ir.cc"],
    hdrs = ["ir.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":bazel_types",
        ":ir_cc_proto",
        "//common:strong_int",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:cord",
        "@abseil-cpp//absl/strings:str_format",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "bazel_label",
    srcs = ["bazel_label.rs"],
    visibility = ["//:__subpackages__"],
)

rust_library(
    name = "ir",
    srcs = [
        "ir.rs",
    ],
    rustc_flags = _PROFILING_RUSTC_FLAGS,
    visibility = ["//:__subpackages__"],
    deps = [
        ":bazel_label",
        "//common:arc_anyhow",
        "//common:code_gen_utils",
        "//common:crubit_feature",
        "//common:error_report",
        "@crate_index//:flagset",  # v0_4
        "@crate_index//:itertools",  # v0_13
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
        "@crate_index//:syn",  # v1
    ],
)

crubit_rust_test(
    name = "rs_ir_test",
    crate = ":ir",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "ir_testing",
    testonly = 1,
    srcs = ["ir_testing.rs"],
    visibility = [
        "//cargo:__subpackages__",
        "//rs_bindings_from_cc/generate_bindings:__subpackages__",
    ],
    deps = [
        ":ir",
        "//common:arc_anyhow",
        "//common:crubit_feature",
        "//common:ffi_types",
        "//common:multiplatform_testing",
        "@crate_index//:flagset",  # v0_4
        "@crate_index//:itertools",  # v0_13
        "@protobuf//rust:protobuf",
    ],
)

crubit_rust_test(
    name = "ir_testing_test",
    crate = ":ir_testing",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        ":ir_matchers",
        ":ir_testing",
        "@crate_index//:googletest",
        "@crate_index//:quote",  # v1
    ],
)

cc_library(
    name = "ir_from_cc",
    srcs = ["ir_from_cc.cc"],
    hdrs = ["ir_from_cc.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline_flags",
        ":decl_importer",
        ":frontend_action",
        ":profiling",
        "//common:status_macros",
        "//common:string_view_conversion",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:serialization",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "ir_from_cc_dependency",
    testonly = 1,
    srcs = ["ir_from_cc_dependency.cc"],
    hdrs = ["ir_from_cc_dependency.h"],
    deps = [
        ":bazel_types",
        ":cc_ir",
        ":cmdline_flags",
        ":ir_from_cc",
        "//common:cc_ffi_types",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_library(
    name = "proto_from_cc",
    testonly = 1,
    srcs = ["proto_from_cc.cc"],
    deps = [
        ":cc_ir",
        ":cmdline_flags",
        ":ir_from_cc_dependency",
        "//common:cc_ffi_types",
        "@abseil-cpp//absl/status:statusor",
        "@llvm-project//llvm:Support",
    ],
    alwayslink = 1,
)

multiplatform_rust_test(
    name = "ir_from_cc_test",
    timeout = "long",
    srcs = ["ir_from_cc_test.rs"],
    tags = [
        "not_run:arm",
        "not_run:mac",
    ],
    deps = [
        ":ir",
        ":ir_matchers",
        ":ir_testing",
        "//common:arc_anyhow",
        "//common:multiplatform_testing",
        "@crate_index//:googletest",
        "@crate_index//:itertools",  # v0_13
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
    ],
)

cc_library(
    name = "src_code_gen_ffi",
    srcs = [
        "src_code_gen_ffi.cc",
        "src_code_gen_ffi_serialized_proto.cc",
    ],
    hdrs = ["src_code_gen_ffi.h"],
    deps = [
        ":profiling",
        "//common:cc_ffi_types",
        "//rs_bindings_from_cc/generate_bindings:generate_bindings_cc_proto",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

crubit_cc_test(
    name = "src_code_gen_ffi_test",
    srcs = ["src_code_gen_ffi_test.cc"],
    deps = [
        ":cmdline",  # buildcleaner: keep
        ":cmdline_flags",
        ":src_code_gen_ffi",
        "//net/proto2/contrib/parse_proto:parse_text_proto",
        "//rs_bindings_from_cc/generate_bindings:cc_api_ffi",  # buildcleaner: keep
        "//rs_bindings_from_cc/generate_bindings:generate_bindings_cc_proto",  # buildcleaner: keep
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/flags:flag",
    ],
)

cc_library(
    name = "src_code_gen",
    srcs = ["src_code_gen.cc"],
    hdrs = ["src_code_gen.h"],
    deps = [
        ":cc_ir",
        ":profiling",
        ":src_code_gen_ffi",
        "//rs_bindings_from_cc/generate_bindings:cc_api_ffi",  # buildcleaner: keep
        "//rs_bindings_from_cc/generate_bindings:generate_bindings_cc_proto",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "ast_convert",
    srcs = ["ast_convert.cc"],
    hdrs = ["ast_convert.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":ast_util",
        ":cc_ir",
        ":decl_importer",
        ":recording_diagnostic_consumer",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/functional:function_ref",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
    ],
)

cc_library(
    name = "recording_diagnostic_consumer",
    srcs = ["recording_diagnostic_consumer.cc"],
    hdrs = ["recording_diagnostic_consumer.h"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "@abseil-cpp//absl/log",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:str_format",
        "@abseil-cpp//absl/strings:string_view",
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
        "@crate_index//:anyhow",  # v1
        "@crate_index//:itertools",  # v0_13
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",  # v1
    ],
)

crubit_rust_test(
    name = "ir_matchers_test",
    crate = ":ir_matchers",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        "//common:arc_anyhow",
        "//common:multiplatform_testing",
        "@crate_index//:googletest",
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
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "collect_instantiations",
    srcs = ["collect_instantiations.rs"],
    deps = [
        "//common:arc_anyhow",
        "//common:ffi_types",
        "@crate_index//:proc-macro2",
        "@crate_index//:serde_json",  # v1
        "@crate_index//:syn",  # v1
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
        "@crate_index//:googletest",
        "@crate_index//:quote",  # v1
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
        "@abseil-cpp//absl/strings:string_view",
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
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
    ],
)
