"""Parse lifetime annotations in C++ code."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "lifetime",
    srcs = ["lifetime.cc"],
    hdrs = ["lifetime.h"],
    deps = [
        "@abseil-cpp//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_error",
    hdrs = ["lifetime_error.h"],
    deps = [
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "lifetime_test",
    srcs = ["lifetime_test.cc"],
    deps = [
        ":lifetime",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "type_lifetimes",
    srcs = [
        "function_lifetimes.cc",
        "type_lifetimes.cc",
    ],
    hdrs = [
        "function_lifetimes.h",
        "type_lifetimes.h",
    ],
    deps = [
        ":lifetime",
        ":lifetime_error",
        ":lifetime_substitutions",
        ":lifetime_symbol_table",
        ":pointee_type",
        "@abseil-cpp//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_annotations",
    srcs = ["lifetime_annotations.cc"],
    hdrs = ["lifetime_annotations.h"],
    deps = [
        ":lifetime",
        ":lifetime_error",
        ":lifetime_symbol_table",
        ":pointee_type",
        ":type_lifetimes",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:lex",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "lifetime_annotations_test",
    srcs = ["lifetime_annotations_test.cc"],
    deps = [
        ":lifetime_annotations",
        ":lifetime_error",
        ":lifetime_symbol_table",
        ":type_lifetimes",
        "//common:status_test_matchers",
        "//lifetime_annotations/test:named_func_lifetimes",
        "//lifetime_annotations/test:run_on_code",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:str_format",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_substitutions",
    srcs = ["lifetime_substitutions.cc"],
    hdrs = ["lifetime_substitutions.h"],
    deps = [
        ":lifetime",
        "@abseil-cpp//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "lifetime_substitutions_test",
    srcs = ["lifetime_substitutions_test.cc"],
    deps = [
        ":lifetime",
        ":lifetime_substitutions",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "lifetime_symbol_table",
    srcs = ["lifetime_symbol_table.cc"],
    hdrs = ["lifetime_symbol_table.h"],
    deps = [
        ":lifetime",
        "@llvm-project//llvm:Support",
    ],
)

crubit_cc_test(
    name = "lifetime_symbol_table_test",
    srcs = ["lifetime_symbol_table_test.cc"],
    deps = [
        ":lifetime",
        ":lifetime_symbol_table",
        "//testing/base/public:gunit_main",
    ],
)

cc_library(
    name = "pointee_type",
    srcs = ["pointee_type.cc"],
    hdrs = ["pointee_type.h"],
    deps = [
        "@llvm-project//clang:ast",
    ],
)
