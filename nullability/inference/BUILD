# Inference of nullability annotations

load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "collect_evidence",
    srcs = ["collect_evidence.cc"],
    hdrs = ["collect_evidence.h"],
    deps = [
        ":inference_cc_proto",
        "@absl//absl/log:check",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:type_nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:index",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "collect_evidence_test",
    srcs = ["collect_evidence_test.cc"],
    deps = [
        ":collect_evidence",
        ":inference_cc_proto",
        "@absl//absl/log:check",
        "//nullability:proto_matchers",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//clang/unittests:dataflow_testing_support",
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
