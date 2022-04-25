"""Importers for various types of `Decl`s"""

package(default_visibility = ["//rs_bindings_from_cc:__pkg__"])

cc_library(
    name = "class_template",
    srcs = ["class_template.cc"],
    hdrs = ["class_template.h"],
    deps = [
        "//rs_bindings_from_cc:decl_importer",
        "@llvm///clang:ast",
    ],
)

cc_library(
    name = "cxx_record",
    srcs = ["cxx_record.cc"],
    hdrs = ["cxx_record.h"],
    deps = [
        "//rs_bindings_from_cc:ast_convert",
        "//rs_bindings_from_cc:decl_importer",
        "@llvm///clang:ast",
        "@llvm///clang:sema",
    ],
)

cc_library(
    name = "enum",
    srcs = ["enum.cc"],
    hdrs = ["enum.h"],
    deps = [
        "//rs_bindings_from_cc:decl_importer",
        "@llvm///clang:ast",
    ],
)
