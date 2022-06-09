# Verification for null safety

cc_library(
    name = "pointer_nullability_lattice",
    hdrs = ["pointer_nullability_lattice.h"],
    deps = [
        "@llvm-project//clang:analysis",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_nullability_matchers",
    srcs = ["pointer_nullability_matchers.cc"],
    hdrs = ["pointer_nullability_matchers.h"],
    deps = [
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
    ],
)

cc_library(
    name = "pointer_nullability_analysis",
    srcs = ["pointer_nullability_analysis.cc"],
    hdrs = ["pointer_nullability_analysis.h"],
    deps = [
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        "//common:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:tooling",
    ],
)

cc_test(
    name = "pointer_nullability_analysis_test",
    srcs = ["pointer_nullability_analysis_test.cc"],
    deps = [
        ":pointer_nullability_analysis",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//llvm:gmock",
        "@llvm-project//llvm:gtest",
        "@llvm-project//llvm:gtest_main",
    ],
)
