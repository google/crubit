"""Importers for various types of `Decl`s"""

package(
    default_applicable_licenses = ["//third_party/crubit:license"],
    default_visibility = ["//rs_bindings_from_cc:__pkg__"],
)

cc_library(
    name = "class_template",
    srcs = ["class_template.cc"],
    hdrs = ["class_template.h"],
    deps = [
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "cxx_record",
    srcs = ["cxx_record.cc"],
    hdrs = ["cxx_record.h"],
    deps = [
        "@absl//absl/container:flat_hash_set",
        "@absl//absl/log",
        "@absl//absl/log:check",
        "@absl//absl/log:die_if_null",
        "//rs_bindings_from_cc:ast_convert",
        "//rs_bindings_from_cc:decl_importer",
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
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "friend",
    srcs = ["friend.cc"],
    hdrs = ["friend.h"],
    deps = [
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "function",
    srcs = ["function.cc"],
    hdrs = ["function.h"],
    deps = [
        "@absl//absl/strings",
        "//rs_bindings_from_cc:ast_util",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:sema",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "function_template",
    srcs = ["function_template.cc"],
    hdrs = ["function_template.h"],
    deps = [
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "namespace",
    srcs = ["namespace.cc"],
    hdrs = ["namespace.h"],
    deps = [
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "typedef_name",
    srcs = ["typedef_name.cc"],
    hdrs = ["typedef_name.h"],
    deps = [
        "@absl//absl/log:check",
        "//rs_bindings_from_cc:decl_importer",
        "//rs_bindings_from_cc:known_types_map",
        "@llvm-project//clang:ast",
    ],
)

cc_test(
    name = "override_final_test",
    srcs = ["override_final_test.cc"],
    deps = ["@com_google_googletest//:gtest_main"],
)
