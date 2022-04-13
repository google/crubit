# Common libraries used in multiple Crubit tools.

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)

licenses(["notice"])

package(default_visibility = ["//third_party/crubit:__subpackages__"])

cc_library(
    name = "file_io",
    srcs = ["file_io.cc"],
    hdrs = ["file_io.h"],
    deps = [
        "@absl//status:statusor",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "ffi_types",
    srcs = ["ffi_types.cc"],
    hdrs = ["ffi_types.h"],
    visibility = ["//third_party/crubit:__subpackages__"],
    deps = [
        ":rs_ffi_types",  # buildcleaner: keep
        "@absl//strings",
    ],
)

rust_library(
    name = "rs_ffi_types",
    srcs = ["ffi_types.rs"],
    crate_name = "ffi_types",
    visibility = ["//third_party/crubit:__subpackages__"],
)

rust_test(
    name = "rs_ffi_types_test",
    crate = ":rs_ffi_types",
)

cc_library(
    name = "check",
    hdrs = ["check.h"],
    deps = [
        "@absl//base:core_headers",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "strong_int",
    hdrs = ["strong_int.h"],
    deps = [
        "@absl//base:core_headers",
        "@absl//meta:type_traits",
    ],
)

cc_library(
    name = "string_type",
    hdrs = ["string_type.h"],
    deps = [
        "@absl//container:flat_hash_set",
        "@absl//flags:marshalling",
        "@absl//meta:type_traits",
        "@absl//strings",
    ],
)

cc_library(
    name = "status_macros",
    hdrs = ["status_macros.h"],
    deps = [
        "@absl//base:core_headers",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//types:source_location",
    ],
)

rust_library(
    name = "token_stream_printer",
    srcs = ["token_stream_printer.rs"],
    data = [
        "@rust//toolchains/nightly:bin/rustfmt",
        "@rustfmt//:rustfmt.toml",
    ],
    deps = [
        "@crates_io//anyhow/v1:anyhow",
        "@crates_io//proc_macro2/v1:proc_macro2",
    ],
)

rust_test(
    name = "token_stream_printer_test",
    crate = ":token_stream_printer",
    data = [
        "@rust//toolchains/nightly:bin/rustfmt",
        "@rustfmt//:rustfmt.toml",
    ],
    deps = [
        "@crates_io//quote/v1:quote",
    ],
)
