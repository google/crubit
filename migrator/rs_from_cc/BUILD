"""Generates equivalent Rust code from C++ code."""

licenses(["notice"])

cc_binary(
    name = "rs_from_cc",
    srcs = ["rs_from_cc.cc"],
    visibility = ["//visibility:public"],
    deps = [
        ":rs_from_cc_lib",
        "@absl//flags:flag",
        "@absl//flags:parse",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "//common:check",
        "//common:file_io",
        "@llvm///llvm:Support",
        "@rust//support:rust_okay_here",
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
        "@llvm///clang:ast",
        "@llvm///clang:frontend",
    ],
)

cc_library(
    name = "ast_consumer",
    srcs = ["ast_consumer.cc"],
    hdrs = ["ast_consumer.h"],
    deps = [
        ":converter",
        "//common:check",
        "@llvm///clang:ast",
        "@llvm///clang:frontend",
    ],
)

cc_library(
    name = "converter",
    srcs = ["converter.cc"],
    hdrs = ["converter.h"],
    deps = [
        "@absl//container:flat_hash_map",
        "@absl//container:flat_hash_set",
        "@absl//status:statusor",
        "@absl//strings",
        "@absl//types:span",
        "//lifetime_annotations",
        "@llvm///clang:ast",
        "@llvm///clang:basic",
        "@llvm///clang:sema",
        "//third_party/re2",
    ],
)

cc_test(
    name = "rs_from_cc_test",
    srcs = ["rs_from_cc_lib_test.cc"],
    deps = [
        ":rs_from_cc_lib",
        "//testing/base/public:gunit_main",
        "@absl//status",
        "@absl//strings",
        "@llvm///clang:ast",
    ],
)

cc_library(
    name = "rs_from_cc_lib",
    srcs = ["rs_from_cc_lib.cc"],
    hdrs = ["rs_from_cc_lib.h"],
    deps = [
        ":converter",
        ":frontend_action",
        "@absl//container:flat_hash_map",
        "@absl//status",
        "@absl//status:statusor",
        "@absl//strings",
        "@absl//types:span",
        "@llvm///clang:basic",
        "@llvm///clang:frontend",
        "@llvm///clang:tooling",
    ],
)
