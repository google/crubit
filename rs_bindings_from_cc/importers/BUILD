"""Importers for various types of `Decl`s"""

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//rs_bindings_from_cc:__pkg__"],
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
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_convert",
        "//rs_bindings_from_cc:bazel_types",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/log",
        "@absl//absl/log:check",
        "@absl//absl/log:die_if_null",
        "@absl//absl/status",
        "@absl//absl/strings",
        "@absl//absl/strings:string_view",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "enum",
    srcs = ["enum.cc"],
    hdrs = ["enum.h"],
    deps = [
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/algorithm:container",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "friend",
    srcs = ["friend.cc"],
    hdrs = ["friend.h"],
    deps = [
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "function",
    srcs = ["function.cc"],
    hdrs = ["function.h"],
    deps = [
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:lifetime_error",
        "//lifetime_annotations:lifetime_symbol_table",
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:bazel_types",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/log:check",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
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
    ],
)

cc_library(
    name = "namespace",
    srcs = ["namespace.cc"],
    hdrs = ["namespace.h"],
    deps = [
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "type_alias",
    srcs = ["type_alias.cc"],
    hdrs = ["type_alias.h"],
    deps = [
        "//lifetime_annotations:type_lifetimes",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "type_map_override",
    srcs = ["type_map_override.cc"],
    hdrs = ["type_map_override.h"],
    deps = [
        "//common:status_macros",
        "//rs_bindings_from_cc:cc_ir",
        "//rs_bindings_from_cc:decl_importer",
        "@absl//absl/status",
        "@absl//absl/status:statusor",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "override_final_test",
    srcs = ["override_final_test.cc"],
    deps = ["@com_google_googletest//:gtest_main"],
)
