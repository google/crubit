# Inference of nullability annotations

load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "collect_evidence",
    srcs = ["collect_evidence.cc"],
    hdrs = ["collect_evidence.h"],
    deps = [
        ":inferable",
        ":inference_cc_proto",
        ":slot_fingerprint",
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:type_nullability",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
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
        ":slot_fingerprint",
        "//nullability/test:headers_for_test",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "merge",
    srcs = ["merge.cc"],
    hdrs = ["merge.h"],
    deps = [
        ":inference_cc_proto",
        "@absl//absl/log:check",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "merge_test",
    srcs = ["merge_test.cc"],
    deps = [
        ":inference_cc_proto",
        ":merge",
        "//nullability:proto_matchers",
        "//third_party/protobuf",
        "@absl//absl/log:check",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "inferable",
    srcs = ["inferable.cc"],
    hdrs = ["inferable.h"],
    deps = [
        "//nullability:type_nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
    ],
)

cc_test(
    name = "inferable_test",
    srcs = ["inferable_test.cc"],
    deps = [
        ":inferable",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "infer_tu",
    srcs = ["infer_tu.cc"],
    hdrs = ["infer_tu.h"],
    deps = [
        ":collect_evidence",
        ":inference_cc_proto",
        ":merge",
        ":slot_fingerprint",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "slot_fingerprint",
    srcs = ["slot_fingerprint.cc"],
    hdrs = ["slot_fingerprint.h"],
    deps = ["@llvm-project//llvm:Support"],
)

cc_library(
    name = "eligible_ranges",
    srcs = ["eligible_ranges.cc"],
    hdrs = ["eligible_ranges.h"],
    deps = [
        ":inference_cc_proto",
        "//nullability:type_nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:index",
        "@llvm-project//clang:transformer",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "eligible_ranges_test",
    srcs = ["eligible_ranges_test.cc"],
    deps = [
        ":eligible_ranges",
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:TestingAnnotations",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "slot_fingerprint_test",
    srcs = ["slot_fingerprint_test.cc"],
    deps = [
        ":slot_fingerprint",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "infer_tu_test",
    srcs = ["infer_tu_test.cc"],
    deps = [
        ":infer_tu",
        ":inference_cc_proto",
        "//nullability:proto_matchers",
        "//nullability/test:headers_for_test",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:index",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_binary(
    name = "infer_tu_main",
    srcs = ["infer_tu_main.cc"],
    deps = [
        ":infer_tu",
        ":inference_cc_proto",
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:index",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:Support",
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
