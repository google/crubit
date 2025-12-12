"""Common libraries used in multiple Crubit tools."""

load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
    "rust_test",
)
load("//common/bazel_support:external_binaries.bzl", "EXTERNAL_BINARIES")
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
        "@crates//:anyhow",  # v1
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
        "@abseil-cpp//absl/base:nullability",
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
        ":string_view_conversion",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:testing",
    ],
)

crubit_rust_test(
    name = "arc_anyhow_test",
    crate = ":arc_anyhow",
    deps = [
        "@crates//:googletest",
    ],
)

rust_library(
    name = "code_gen_utils",
    srcs = ["code_gen_utils.rs"],
    deps = [
        ":arc_anyhow",
        "//common:dyn_format",
        "@crates//:heck",  # v0_5
        "@crates//:phf",  # v0_11
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
        "@crates//:syn",  # v1
        "@crates//:unicode-ident",
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
        "@crates//:googletest",
        "@crates//:itertools",  # v0_13
    ],
)

rust_library(
    name = "crubit_feature",
    srcs = ["crubit_feature.rs"],
    deps = [
        "@crates//:flagset",  # v0_4
        "@crates//:serde",  # v1
    ],
)

rust_library(
    name = "dyn_format",
    srcs = ["dyn_format.rs"],
    deps = [
        "@crates//:anyhow",  # v1
    ],
)

rust_test(
    name = "dyn_format_test",
    srcs = ["dyn_format_test.rs"],
    deps = [
        ":dyn_format",
        "@crates//:googletest",
    ],
)

crubit_rust_test(
    name = "crubit_feature_test",
    crate = ":crubit_feature",
    deps = [
        "@crates//:googletest",
        "@crates//:serde_json",  # v1
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

cc_library(
    name = "external_binaries_cc",
    hdrs = ["external_binaries.h"],
    deps = ["@abseil-cpp//absl/strings:string_view"],
)

rust_library(
    name = "external_binaries",
    srcs = ["external_binaries.rs"],
    rustc_env = EXTERNAL_BINARIES,
)

rust_library(
    name = "interner",
    srcs = ["interner.rs"],
    deps = [
        "@crates//:bumpalo",  # v3
    ],
)

rust_test(
    name = "interner_test",
    srcs = ["interner_test.rs"],
    deps = [
        ":interner",
        "@crates//:bumpalo",  # v3
        "@crates//:googletest",
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
        "@crates//:googletest",
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
        "@crates//:googletest",
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
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
        "@crates//:syn",  # v1
    ],
)

crubit_rust_test(
    name = "item_exists_test",
    srcs = ["item_exists_test.rs"],
    proc_macro_deps = [
        ":item_exists",
    ],
    deps = [
        "@crates//:googletest",
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
        "@crates//:anyhow",  # v1
        "@crates//:indenter",  # v0_3
        "@crates//:proc-macro2",
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
        "@crates//:googletest",
        "@crates//:quote",  # v1
    ],
)

rust_library(
    name = "token_stream_matchers_fastpath",
    testonly = 1,
    srcs = ["token_stream_matchers_fastpath.rs"],
    visibility = ["//cargo:__subpackages__"],
    deps = [
        ":token_stream_printer",
        "@crates//:anyhow",  # v1
        "@crates//:proc-macro2",
    ],
)

rust_library(
    name = "token_stream_printer",
    srcs = ["token_stream_printer.rs"],
    data = [
    ],
    deps = [
        ":external_binaries",
        ":ffi_types",
        "@crates//:anyhow",  # v1
        "@crates//:proc-macro2",
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
        "@crates//:googletest",
        "@crates//:quote",  # v1
        "@crates//:tempfile",  # v3
    ],
)

rust_library(
    name = "kythe_metadata",
    srcs = ["kythe_metadata.rs"],
    deps = [
        ":token_stream_printer",
        "@crates//:anyhow",  # v1
        "@crates//:base64",  # v0_22
        "@crates//:serde",  # v1
        "@crates//:serde_json",  # v1
    ],
)

crubit_rust_test(
    name = "kythe_metadata_test",
    crate = ":kythe_metadata",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        "@crates//:googletest",
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
        "@crates//:anyhow",  # v1
    ],
)

crubit_rust_test(
    name = "errors_test",
    srcs = ["errors_test.rs"],
    deps = [
        ":arc_anyhow",
        ":error_report",
        ":errors",
        "@crates//:googletest",
        "@crates//:serde_json",  # v1
    ],
)

rust_library(
    name = "error_report",
    srcs = ["error_report.rs"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":arc_anyhow",
        "@crates//:anyhow",  # v1
        "@crates//:regex",  # v1
        "@crates//:serde",  # v1
        "@crates//:serde_json",  # v1
    ],
)

crubit_rust_test(
    name = "error_report_test",
    crate = ":error_report",
    deps = [
        "@crates//:googletest",
        "@crates//:serde_json",  # v1
    ],
)

rust_library(
    name = "crubit_abi_type",
    srcs = ["crubit_abi_type.rs"],
    visibility = [
        "//:__subpackages__",
        "//cargo:__subpackages__",
    ],
    deps = [
        "@crates//:proc-macro2",
        "@crates//:quote",  # v1
    ],
)

crubit_rust_test(
    name = "crubit_abi_type_test",
    crate = ":crubit_abi_type",
    deps = [
        "@crates//:googletest",
    ],
)
