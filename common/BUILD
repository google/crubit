"""Common libraries used in multiple Crubit tools."""

load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

exports_files([
    "LICENSE_HEADER",
    "golden_test.sh",
])

rust_library(
    name = "arc_anyhow",
    srcs = ["arc_anyhow.rs"],
    deps = [
        "@crate_index//:anyhow",
    ],
)

cc_library(
    name = "annotation_reader",
    srcs = ["annotation_reader.cc"],
    hdrs = ["annotation_reader.h"],
    deps = [
        ":status_macros",
        ":string_view_conversion",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "annotation_reader_test",
    srcs = ["annotation_reader_test.cc"],
    deps = [
        ":annotation_reader",
        ":status_test_matchers",
        "@abseil-cpp//absl/strings:string_view",
        "@googletest//:gtest_main",
        "@llvm-project//clang:testing",
    ],
)

crubit_rust_test(
    name = "arc_anyhow_test",
    crate = ":arc_anyhow",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "code_gen_utils",
    srcs = ["code_gen_utils.rs"],
    deps = [
        ":arc_anyhow",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
        "@crate_index//:unicode-ident",
    ],
)

crubit_rust_test(
    name = "code_gen_utils_test",
    crate = ":code_gen_utils",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        ":token_stream_matchers",
        ":token_stream_printer",
        "@crate_index//:googletest",
        "@crate_index//:itertools",
    ],
)

rust_library(
    name = "crubit_feature",
    srcs = ["crubit_feature.rs"],
    deps = [
        "@crate_index//:flagset",
        "@crate_index//:serde",
    ],
)

crubit_rust_test(
    name = "crubit_feature_test",
    crate = ":crubit_feature",
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:serde_json",
    ],
)

cc_library(
    name = "file_io",
    srcs = ["file_io.cc"],
    hdrs = ["file_io.h"],
    deps = [
        ":string_view_conversion",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "cc_ffi_types",
    srcs = ["ffi_types.cc"],
    hdrs = ["ffi_types.h"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":ffi_types",  # buildcleaner: keep
        "@abseil-cpp//absl/strings:string_view",
    ],
)

rust_library(
    name = "interner",
    srcs = ["interner.rs"],
    deps = [
        "@crate_index//:bumpalo",
    ],
)

rust_test(
    name = "interner_test",
    srcs = ["interner_test.rs"],
    deps = [
        ":interner",
        "@crate_index//:bumpalo",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "memoized",
    srcs = ["memoized.rs"],
)

rust_test(
    name = "memoized_test",
    crate = ":memoized",
    deps = [
        "@crate_index//:googletest",
    ],
)

bzl_library(
    name = "multiplatform_testing_bzl",
    srcs = ["multiplatform_testing.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
)

rust_library(
    name = "multiplatform_testing",
    testonly = 1,
    srcs = ["multiplatform_testing.rs"],
    visibility = [
        "//:__subpackages__",
    ],
)

rust_library(
    name = "ffi_types",
    srcs = ["ffi_types.rs"],
    visibility = ["//:__subpackages__"],
)

crubit_rust_test(
    name = "ffi_types_test",
    crate = ":ffi_types",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_proc_macro(
    name = "item_exists",
    testonly = 1,
    srcs = ["item_exists.rs"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

crubit_rust_test(
    name = "item_exists_test",
    srcs = ["item_exists_test.rs"],
    proc_macro_deps = [
        ":item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = ["@abseil-cpp//absl/hash"],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "@abseil-cpp//absl/flags:marshalling",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_library(
    name = "status_macros",
    hdrs = ["status_macros.h"],
    deps = [
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
    ],
)

rust_library(
    name = "token_stream_matchers",
    testonly = 1,
    srcs = ["token_stream_matchers.rs"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":token_stream_matchers_fastpath",
        ":token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

crubit_rust_test(
    name = "token_stream_matchers_test",
    timeout = "short",
    crate = ":token_stream_matchers",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:quote",
    ],
)

rust_library(
    name = "token_stream_matchers_fastpath",
    testonly = 1,
    srcs = ["token_stream_matchers_fastpath.rs"],
    visibility = ["//visibility:private"],
    deps = [
        ":token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

rust_library(
    name = "token_stream_printer",
    srcs = ["token_stream_printer.rs"],
    data = [
    ],
    deps = [
        "//common:ffi_types",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

crubit_rust_test(
    name = "token_stream_printer_test",
    crate = ":token_stream_printer",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:quote",
        "@crate_index//:tempfile",
    ],
)

cc_library(
    name = "test_utils",
    testonly = True,
    srcs = ["test_utils.cc"],
    hdrs = ["test_utils.h"],
    deps = [
        ":file_io",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@googletest//:gtest",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "status_test_matchers",
    testonly = True,
    hdrs = ["status_test_matchers.h"],
    deps = [
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@googletest//:gtest",
    ],
)

cc_library(
    name = "rust_allocator_shims",
    srcs = ["rust_allocator_shims.c"],
    visibility = ["@rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools//:__subpackages__"],
)

cc_library(
    name = "string_view_conversion",
    hdrs = ["string_view_conversion.h"],
    deps = [
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//llvm:Support",
    ],
)

rust_library(
    name = "errors",
    srcs = ["errors.rs"],
    deps = [
        ":arc_anyhow",
        ":error_report",
        "@crate_index//:anyhow",
    ],
)

crubit_rust_test(
    name = "errors_test",
    srcs = ["errors_test.rs"],
    deps = [
        ":error_report",
        ":errors",
        "@crate_index//:googletest",
        "@crate_index//:serde_json",
    ],
)

rust_library(
    name = "error_report",
    srcs = ["error_report.rs"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":arc_anyhow",
        "@crate_index//:anyhow",
        "@crate_index//:regex",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

crubit_rust_test(
    name = "error_report_test",
    crate = ":error_report",
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:serde_json",
    ],
)

rust_library(
    name = "crubit_abi_type",
    srcs = ["crubit_abi_type.rs"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

crubit_rust_test(
    name = "crubit_abi_type_test",
    crate = ":crubit_abi_type",
    deps = [
        "@crate_index//:googletest",
    ],
)
