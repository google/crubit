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
