# Verification for null safety

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "pointer_nullability_lattice",
    hdrs = ["pointer_nullability_lattice.h"],
    deps = [
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
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
    visibility = ["//nullability_verification/test:__pkg__"],
    deps = [
        ":pointer_nullability",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
    ],
)

cc_library(
    name = "pointer_nullability_diagnosis",
    srcs = ["pointer_nullability_diagnosis.cc"],
    hdrs = ["pointer_nullability_diagnosis.h"],
    visibility = ["//nullability_verification/test:__pkg__"],
    deps = [
        ":pointer_nullability",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_nullability",
    srcs = [
        "pointer_nullability.cc",
        "type_nullability.cc",
    ],
    hdrs = ["pointer_nullability.h"],
    deps = [
        ":pointer_nullability_lattice",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "pointer_nullability_test",
    srcs = ["pointer_nullability_test.cc"],
    deps = [
        ":pointer_nullability",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)
