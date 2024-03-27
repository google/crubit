"""Common libraries used in multiple Crubit tools."""

load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_proc_macro",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

rust_library(
    name = "arc_anyhow",
    srcs = ["arc_anyhow.rs"],
    deps = [
        "@crate_index//:anyhow",
    ],
)

crubit_rust_test(
    name = "arc_anyhow_test",
    crate = ":arc_anyhow",
)

rust_library(
    name = "code_gen_utils",
    srcs = ["code_gen_utils.rs"],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:itertools",
        "@crate_index//:once_cell",
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
    ],
    deps = [
        ":token_stream_matchers",
        ":token_stream_printer",
    ],
)

cc_library(
    name = "file_io",
    srcs = ["file_io.cc"],
    hdrs = ["file_io.h"],
    deps = [
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
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
        "@absl//absl/strings",
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
    deps = [
        "@crate_index//:once_cell",
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
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = ["@absl//absl/hash"],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "@absl//absl/flags:marshalling",
        "@absl//absl/strings",
    ],
)

cc_library(
    name = "status_macros",
    hdrs = ["status_macros.h"],
    deps = [
        "@absl//absl/base:core_headers",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
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
        ":token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

crubit_rust_test(
    name = "token_stream_matchers_test",
    crate = ":token_stream_matchers",
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
    ],
    deps = ["@crate_index//:quote"],
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
    ],
    deps = [
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
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "@com_google_googletest//:gtest",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "status_test_matchers",
    testonly = True,
    hdrs = ["status_test_matchers.h"],
    deps = [
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@com_google_googletest//:gtest",
    ],
)

cc_library(
    name = "rust_allocator_shims",
    srcs = ["rust_allocator_shims.c"],
    visibility = ["@rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools//:__subpackages__"],
)
