# Common libraries used in multiple Crubit tools.

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)

licenses(["notice"])

package(default_visibility = ["//:__subpackages__"])

rust_library(
    name = "arc_anyhow",
    srcs = ["arc_anyhow.rs"],
    deps = [
        "@crate_index//:anyhow",
    ],
)

rust_test(
    name = "arc_anyhow_test",
    crate = ":arc_anyhow",
)

rust_library(
    name = "code_gen_utils",
    srcs = ["code_gen_utils.rs"],
    deps = [
        "@crate_index//:anyhow",
        "@crate_index//:once_cell",
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
    ],
)

rust_test(
    name = "code_gen_utils_test",
    crate = ":code_gen_utils",
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
        "@absl//absl/status:statusor",
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

rust_library(
    name = "ffi_types",
    srcs = ["ffi_types.rs"],
    visibility = ["//:__subpackages__"],
)

rust_test(
    name = "ffi_types_test",
    crate = ":ffi_types",
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = [
        "@absl//absl/base:core_headers",
        "@absl//absl/meta:type_traits",
    ],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/flags:marshalling",
        "@absl//absl/meta:type_traits",
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
    visibility = ["//:__subpackages__"],
    deps = [
        ":token_stream_printer",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

rust_test(
    name = "token_stream_matchers_test",
    crate = ":token_stream_matchers",
    deps = [
        "//common:rust_allocator_shims",
        "@crate_index//:quote",
    ],
)

rust_library(
    name = "token_stream_printer",
    srcs = ["token_stream_printer.rs"],
    data = [
    ],
    deps = [
        ":token_stream_printer_impl",
        "//common:ffi_types",
        "@crate_index//:anyhow",
        "@crate_index//:proc-macro2",
    ],
)

rust_test(
    name = "token_stream_printer_test",
    crate = ":token_stream_printer",
    deps = [
        "@crate_index//:quote",
        "@crate_index//:tempfile",
    ],
)

cc_library(
    name = "token_stream_printer_impl",
    srcs = ["token_stream_printer_impl.cc"],
    hdrs = ["token_stream_printer_impl.h"],
    deps = [
        "//common:cc_ffi_types",
        "@absl//absl/log",
        "@absl//absl/log:check",
        "@llvm-project//clang:format",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "token_stream_printer_impl_test",
    srcs = ["token_stream_printer_impl_test.cc"],
    deps = [
        ":token_stream_printer_impl",
        "//common:cc_ffi_types",
        "//common:rust_allocator_shims",
        "@com_google_googletest//:gtest_main",
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
)
