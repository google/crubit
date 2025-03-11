# Verification and Inference for null safety

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "ast_helpers",
    hdrs = ["ast_helpers.h"],
    visibility = ["//nullability/inference:__pkg__"],
    deps = ["@llvm-project//clang:ast"],
)

cc_library(
    name = "forwarding_functions",
    srcs = ["forwarding_functions.cc"],
    hdrs = ["forwarding_functions.h"],
    visibility = ["//nullability/inference:__pkg__"],
    deps = [
        "@abseil-cpp//absl/base:nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_nullability_lattice",
    srcs = ["pointer_nullability_lattice.cc"],
    hdrs = ["pointer_nullability_lattice.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        ":pointer_nullability",
        ":type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/container:flat_hash_map",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "pointer_nullability_lattice_test",
    srcs = ["pointer_nullability_lattice_test.cc"],
    deps = [
        ":pointer_nullability_lattice",
        "@abseil-cpp//absl/base:nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "pointer_nullability_matchers",
    srcs = ["pointer_nullability_matchers.cc"],
    hdrs = ["pointer_nullability_matchers.h"],
    visibility = ["//visibility:private"],
    deps = [
        ":type_nullability",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
    ],
)

cc_test(
    name = "pointer_nullability_matchers_test",
    srcs = ["pointer_nullability_matchers_test.cc"],
    deps = [
        ":pointer_nullability_matchers",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "pointer_nullability_analysis",
    srcs = ["pointer_nullability_analysis.cc"],
    hdrs = ["pointer_nullability_analysis.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        ":ast_helpers",
        ":macro_arg_capture",
        ":pointer_nullability",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        ":pragma",
        ":type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "pointer_nullability_analysis_test",
    srcs = ["pointer_nullability_analysis_test.cc"],
    deps = [
        ":pointer_nullability",
        ":pointer_nullability_analysis",
        ":pointer_nullability_lattice",
        ":pragma",
        ":type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "pointer_nullability_diagnosis",
    srcs = ["pointer_nullability_diagnosis.cc"],
    hdrs = ["pointer_nullability_diagnosis.h"],
    visibility = ["//nullability/test:__pkg__"],
    deps = [
        ":forwarding_functions",
        ":pointer_nullability",
        ":pointer_nullability_analysis",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        ":pragma",
        ":type_nullability",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:transformer",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_nullability",
    srcs = ["pointer_nullability.cc"],
    hdrs = ["pointer_nullability.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        ":type_nullability",
        "@abseil-cpp//absl/base:nullability",
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
        ":type_nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "type_nullability",
    srcs = ["type_nullability.cc"],
    hdrs = ["type_nullability.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        ":pragma",
        ":type_and_maybe_loc_visitor",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "type_nullability_test",
    srcs = ["type_nullability_test.cc"],
    deps = [
        ":pragma",
        ":type_nullability",
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:lex",
        "@llvm-project//clang:testing",
        "@llvm-project//clang:transformer",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingAnnotations",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "pragma",
    srcs = ["pragma.cc"],
    hdrs = ["pragma.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        "@llvm-project//clang:basic",
        "@llvm-project//clang:lex",
        "@llvm-project//llvm:Support",
    ],
)

cc_test(
    name = "pragma_test",
    srcs = ["pragma_test.cc"],
    deps = [
        ":pragma",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "proto_matchers",
    testonly = 1,
    srcs = ["proto_matchers.cc"],
    hdrs = ["proto_matchers.h"],
    visibility = [":__subpackages__"],
    deps = [
        "//third_party/protobuf",
        "@abseil-cpp//absl/base:nullability",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
    ],
)

cc_library(
    name = "type_and_maybe_loc_visitor",
    hdrs = ["type_and_maybe_loc_visitor.h"],
    deps = [
        "@llvm-project//clang:ast",
        "@llvm-project//clang:type_nodes_gen",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "macro_arg_capture",
    hdrs = ["macro_arg_capture.h"],
    visibility = ["//nullability/inference:__pkg__"],
    deps = ["@llvm-project//llvm:Support"],
)

cc_library(
    name = "loc_filter",
    srcs = ["loc_filter.cc"],
    hdrs = ["loc_filter.h"],
    visibility = ["//nullability/inference:__pkg__"],
    deps = [
        "@abseil-cpp//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "annotations",
    hdrs = ["annotations.h"],
    visibility = ["//nullability/inference:__pkg__"],
    deps = ["@llvm-project//llvm:Support"],
)

cc_test(
    name = "loc_filter_test",
    srcs = ["loc_filter_test.cc"],
    deps = [
        ":loc_filter",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:testing",
        "@llvm-project//third-party/unittest:gtest",
    ],
)

cc_test(
    name = "pointer_nullability_analysis_benchmark",
    timeout = "long",
    srcs = ["pointer_nullability_analysis_benchmark.cc"],
    tags = ["benchmark"],
    deps = [
        ":pointer_nullability_analysis",
        ":pointer_nullability_diagnosis",
        ":pragma",
        "//third_party/benchmark",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
    ],
)
