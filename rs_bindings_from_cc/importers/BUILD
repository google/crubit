"""Importers for various types of `Decl`s"""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_cc//cc:cc_test.bzl", "cc_test")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

cc_library(
    name = "class_template",
    srcs = ["class_template.cc"],
    hdrs = ["class_template.h"],
    deps = [
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "cxx_record",
    srcs = ["cxx_record.cc"],
    hdrs = ["cxx_record.h"],
    deps = [
        "//common:annotation_reader",
        "//common:check",
        "//common:status_macros",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_convert",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:bazel_types",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "enum",
    srcs = ["enum.cc"],
    hdrs = ["enum.h"],
    deps = [
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:bazel_types",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "enum_constant",
    srcs = ["enum_constant.cc"],
    hdrs = ["enum_constant.h"],
    deps = [
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "var",
    srcs = ["var.cc"],
    hdrs = ["var.h"],
    deps = [
        "//common:check",
        "//common:status_macros",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:clang_compat_macros",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "friend",
    srcs = ["friend.cc"],
    hdrs = ["friend.h"],
    deps = [
        "//common:check",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "function",
    srcs = ["function.cc"],
    hdrs = ["function.h"],
    deps = [
        "//common:annotation_reader",
        "//common:check",
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:lifetime_error",
        "//lifetime_annotations:lifetime_symbol_table",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:clang_compat_macros",
        "//rs_bindings_from_cc:decl_importer",
        "//rs_bindings_from_cc:recording_diagnostic_consumer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:lex",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "function_template",
    srcs = ["function_template.cc"],
    hdrs = ["function_template.h"],
    deps = [
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "namespace",
    srcs = ["namespace.cc"],
    hdrs = ["namespace.h"],
    deps = [
        "//common:check",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "type_alias",
    srcs = ["type_alias.cc"],
    hdrs = ["type_alias.h"],
    deps = [
        "//common:check",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "existing_rust_type",
    srcs = ["existing_rust_type.cc"],
    hdrs = ["existing_rust_type.h"],
    deps = [
        "//common:annotation_reader",
        "//common:status_macros",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "cxx_record_test",
    srcs = ["cxx_record_test.cc"],
    deps = [
        ":cxx_record",
        "@googletest//:gtest_main",
    ],
)
