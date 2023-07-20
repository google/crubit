"""Generates equivalent Rust code from C++ code."""

package(default_applicable_licenses = ["//:license"])

cc_binary(
    name = "rs_from_cc",
    srcs = ["rs_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":rs_from_cc_lib",
        "//common:file_io",
        "//common:rust_allocator_shims",
        "@absl//absl/flags:flag",
        "@absl//absl/flags:parse",
        "@absl//absl/log:check",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//llvm:Support",
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
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
    ],
)

cc_library(
    name = "converter",
    srcs = ["converter.cc"],
    hdrs = ["converter.h"],
    deps = [
        "//lifetime_annotations",
        "//third_party/re2",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@absl//absl/types:span",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
    ],
)

cc_test(
    name = "rs_from_cc_test",
    srcs = ["rs_from_cc_lib_test.cc"],
    deps = [
        ":rs_from_cc_lib",
        "//common:status_test_matchers",
        "@absl//absl/status",
        "@absl//absl/strings",
        "@com_google_googletest//:gtest_main",
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
