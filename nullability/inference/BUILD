# Inference of nullability annotations

load("@bazel_tools//tools/build_defs/cc:cc_embed_data.bzl", "cc_embed_data")
load("@rules_proto//proto:defs.bzl", "proto_library")
load("//third_party/bazel_rules/rules_cc/cc:cc_library.bzl", "cc_library")
load("//third_party/protobuf/bazel:proto_library.bzl", "proto_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "collect_evidence",
    srcs = ["collect_evidence.cc"],
    hdrs = ["collect_evidence.h"],
    deps = [
        ":inferable",
        ":inference_cc_proto",
        ":slot_fingerprint",
        ":usr_cache",
        "//nullability:ast_helpers",
        "//nullability:forwarding_functions",
        "//nullability:loc_filter",
        "//nullability:macro_arg_capture",
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:pragma",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
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
        ":augmented_test_inputs",
        ":collect_evidence",
        ":inference_cc_proto",
        ":slot_fingerprint",
        ":usr_cache",
        "//nullability:pragma",
        "//nullability:type_nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:testing",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingSupport",
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
        "@abseil-cpp//absl/log:check",
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
        "@abseil-cpp//absl/log:check",
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
        "//nullability:type_nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
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
        ":usr_cache",
        "//nullability:pragma",
        "@abseil-cpp//absl/container:flat_hash_map",
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
        ":inferable",
        ":inference_cc_proto",
        ":usr_cache",
        "//nullability:annotations",
        "//nullability:loc_filter",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:index",
        "@llvm-project//clang:lex",
        "@llvm-project//clang:transformer",
        "@llvm-project//clang-tools-extra/clang-tidy:tidy_framework",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "eligible_ranges_test",
    srcs = ["eligible_ranges_test.cc"],
    deps = [
        ":augmented_test_inputs",
        ":eligible_ranges",
        ":eligible_ranges_for_test",
        ":inference_cc_proto",
        "//nullability:pragma",
        "//nullability:proto_matchers",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
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
        ":augmented_test_inputs",
        ":infer_tu",
        ":inference_cc_proto",
        "//nullability:pragma",
        "//nullability:proto_matchers",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
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
        ":clang_tidy_nullability_replacement_macros",
        ":infer_tu",
        ":inference_cc_proto",
        ":replace_macros",
        "//nullability:pragma",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
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

filegroup(
    name = "replacement_macros_exclude_from_build_cleaner",
    srcs = ["clang_tidy_nullability_replacement_macros.h"],
    tags = ["ignore_srcs"],
    visibility = ["//visibility:private"],
)

cc_embed_data(
    name = "clang_tidy_nullability_replacement_macros",
    srcs = ["clang_tidy_nullability_replacement_macros.h"],
    outs = [
        "ctn_replacement_macros.cc",
        "ctn_replacement_macros.h",
        "ctn_replacement_macros.o",
    ],
)

cc_library(
    name = "replace_macros",
    srcs = ["replace_macros.cc"],
    hdrs = ["replace_macros.h"],
    deps = [
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:lex",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "augmented_test_inputs",
    testonly = True,
    srcs = ["augmented_test_inputs.cc"],
    hdrs = ["augmented_test_inputs.h"],
    deps = [
        ":clang_tidy_nullability_replacement_macros",
        ":replace_macros",
        "//nullability:pragma",
        "//nullability/test:test_headers",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "usr_cache",
    srcs = ["usr_cache.cc"],
    hdrs = ["usr_cache.h"],
    deps = [
        "@llvm-project//clang:ast",
        "@llvm-project//clang:index",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "eligible_ranges_for_test",
    testonly = True,
    hdrs = ["eligible_ranges_for_test.h"],
    deps = [
        ":eligible_ranges",
        "//nullability:type_nullability",
        "@llvm-project//clang:ast",
    ],
)

cc_test(
    name = "replace_macros_test",
    srcs = ["replace_macros_test.cc"],
    deps = [
        ":clang_tidy_nullability_replacement_macros",
        ":replace_macros",
        "//nullability:macro_arg_capture",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
    ],
)
