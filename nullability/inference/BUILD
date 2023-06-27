# Inference of nullability annotations

load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "infer_nullability_constraints",
    srcs = ["infer_nullability_constraints.cc"],
    hdrs = ["infer_nullability_constraints.h"],
    deps = [
        ":inference_cc_proto",
        ":resolve_constraints",
        ":safety_constraint_generator",
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:type_nullability",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "infer_nullability_constraints_test",
    srcs = ["infer_nullability_constraints_test.cc"],
    deps = [
        ":analyze_target_for_test",
        ":infer_nullability_constraints",
        ":inference_cc_proto",
        "//nullability:proto_matchers",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "resolve_constraints",
    srcs = ["resolve_constraints.cc"],
    hdrs = ["resolve_constraints.h"],
    deps = [
        ":inference_cc_proto",
        "//nullability:pointer_nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "safety_constraint_generator",
    srcs = ["safety_constraint_generator.cc"],
    hdrs = ["safety_constraint_generator.h"],
    deps = [
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_lattice",
        "//nullability:pointer_nullability_matchers",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "analyze_target_for_test",
    testonly = True,
    srcs = ["analyze_target_for_test.cc"],
    hdrs = ["analyze_target_for_test.h"],
    deps = [
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "resolve_constraints_test",
    srcs = ["resolve_constraints_test.cc"],
    deps = [
        ":inference_cc_proto",
        ":resolve_constraints",
        "//nullability:pointer_nullability",
        "//nullability:proto_matchers",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "safety_constraint_generator_test",
    srcs = ["safety_constraint_generator_test.cc"],
    deps = [
        ":analyze_target_for_test",
        ":safety_constraint_generator",
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

proto_library(
    name = "inference_proto",
    srcs = ["inference.proto"],
)

cc_proto_library(
    name = "inference_cc_proto",
    deps = [":inference_proto"],
)
