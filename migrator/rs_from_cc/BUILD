"""Generates equivalent Rust code from C++ code."""

licenses(["notice"])

cc_binary(
    name = "rs_from_cc",
    srcs = ["rs_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":rs_from_cc_lib",
        "@absl//absl/flags:flag",
        "@absl//absl/flags:parse",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "//common:check",
        "//common:file_io",
        "@llvm-project//llvm:Support",
        "//common:rust_allocator_shims",
    ],
)

cc_library(
    name = "frontend_action",
    srcs = ["frontend_action.cc"],
    hdrs = ["frontend_action.h"],
    deps = [
        ":ast_consumer",
        ":converter",
        "//lifetime_annotations",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "ast_consumer",
    srcs = ["ast_consumer.cc"],
    hdrs = ["ast_consumer.h"],
    deps = [
        ":converter",
        "//common:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "converter",
    srcs = ["converter.cc"],
    hdrs = ["converter.h"],
    deps = [
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "//lifetime_annotations",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "//third_party/re2",
    ],
)

cc_test(
    name = "rs_from_cc_test",
    srcs = ["rs_from_cc_lib_test.cc"],
    deps = [
        ":rs_from_cc_lib",
        "@com_google_googletest//:gtest_main",
        "@absl//absl/status",
        "@absl//absl/strings",
        "//common:status_test_matchers",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "rs_from_cc_lib",
    srcs = ["rs_from_cc_lib.cc"],
    hdrs = ["rs_from_cc_lib.h"],
    deps = [
        ":converter",
        ":frontend_action",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
    ],
)
