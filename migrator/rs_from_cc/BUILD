"""Generates equivalent Rust code from C++ code."""

package(default_applicable_licenses = ["//:license"])

cc_binary(
    name = "rs_from_cc",
    srcs = ["rs_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":rs_from_cc_lib",
        "//common:file_io",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/flags:parse",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings:string_view",
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
        "@abseil-cpp//absl/log:check",
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
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/container:flat_hash_set",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
        "@re2",
    ],
)

cc_test(
    name = "rs_from_cc_test",
    srcs = ["rs_from_cc_lib_test.cc"],
    deps = [
        ":rs_from_cc_lib",
        "//common:status_test_matchers",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/strings:string_view",
        "@googletest//:gtest_main",
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
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@abseil-cpp//absl/types:span",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
    ],
)
