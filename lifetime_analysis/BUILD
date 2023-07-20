# C++ lifetime inference and verification through static analysis

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "analyze",
    srcs = ["analyze.cc"],
    hdrs = ["analyze.h"],
    deps = [
        ":lifetime_analysis",
        ":lifetime_constraints",
        ":lifetime_lattice",
        ":object",
        ":object_repository",
        ":object_set",
        ":points_to_map",
        ":template_placeholder_support",
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:lifetime_substitutions",
        "//lifetime_annotations:type_lifetimes",
        "@absl//absl/strings",
        "@absl//absl/strings:str_format",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:index",
        "@llvm-project//clang:lex",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "template_placeholder_support",
    srcs = ["template_placeholder_support.cc"],
    hdrs = ["template_placeholder_support.h"],
    deps = [
        "@absl//absl/strings",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:lex",
        "@llvm-project//clang:tooling",
        "@llvm-project//clang:tooling_refactoring",
        "@llvm-project//clang:transformer",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "builtin_lifetimes",
    srcs = ["builtin_lifetimes.cc"],
    hdrs = ["builtin_lifetimes.h"],
    deps = [
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:type_lifetimes",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_analysis",
    srcs = ["lifetime_analysis.cc"],
    hdrs = ["lifetime_analysis.h"],
    deps = [
        ":lifetime_constraints",
        ":lifetime_lattice",
        ":object",
        ":object_repository",
        ":object_set",
        ":points_to_map",
        "//lifetime_annotations:pointee_type",
        "//lifetime_annotations:type_lifetimes",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_constraints",
    srcs = ["lifetime_constraints.cc"],
    hdrs = ["lifetime_constraints.h"],
    deps = [
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:lifetime_substitutions",
        "//lifetime_annotations:pointee_type",
        "//lifetime_annotations:type_lifetimes",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "lifetime_lattice",
    srcs = ["lifetime_lattice.cc"],
    hdrs = ["lifetime_lattice.h"],
    deps = [
        ":lifetime_constraints",
        ":object_set",
        ":points_to_map",
        "@llvm-project//clang:analysis",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "object",
    srcs = ["object.cc"],
    hdrs = ["object.h"],
    deps = [
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:type_lifetimes",
        "@absl//absl/strings",
        "@absl//absl/strings:str_format",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "object_set",
    srcs = ["object_set.cc"],
    hdrs = ["object_set.h"],
    deps = [
        ":object",
        "@absl//absl/strings",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "object_set_test",
    srcs = ["object_set_test.cc"],
    deps = [
        ":object",
        ":object_set",
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations/test:run_on_code",
        "@com_google_googletest//:gtest_main",
        "@llvm-project//clang:ast",
    ],
)

cc_library(
    name = "points_to_map",
    srcs = ["points_to_map.cc"],
    hdrs = ["points_to_map.h"],
    deps = [
        ":object",
        ":object_set",
        "//lifetime_annotations:lifetime",
        "@absl//absl/strings",
        "@absl//absl/strings:str_format",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "points_to_map_test",
    srcs = ["points_to_map_test.cc"],
    deps = [
        ":object",
        ":object_set",
        ":points_to_map",
        "//lifetime_annotations",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations/test:run_on_code",
        "@com_google_googletest//:gtest_main",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
    ],
)

cc_library(
    name = "object_repository",
    srcs = ["object_repository.cc"],
    hdrs = ["object_repository.h"],
    deps = [
        ":builtin_lifetimes",
        ":object",
        ":object_set",
        ":points_to_map",
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:pointee_type",
        "//lifetime_annotations:type_lifetimes",
        "@absl//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_compatibility",
    srcs = ["pointer_compatibility.cc"],
    hdrs = ["pointer_compatibility.h"],
    deps = [
        "//lifetime_annotations:pointee_type",
        "@llvm-project//clang:ast",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "pointer_compatibility_test",
    srcs = ["pointer_compatibility_test.cc"],
    deps = [
        ":pointer_compatibility",
        "//lifetime_annotations",
        "//lifetime_annotations/test:run_on_code",
        "@com_google_googletest//:gtest_main",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//llvm:Support",
    ],
)
