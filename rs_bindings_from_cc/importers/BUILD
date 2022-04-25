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
