# Verification and Inference for null safety

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "pointer_nullability_lattice",
    srcs = ["pointer_nullability_lattice.cc"],
    hdrs = ["pointer_nullability_lattice.h"],
    visibility = [
        "//nullability/inference:__pkg__",
        "//nullability/test:__pkg__",
    ],
    deps = [
        ":type_nullability",
        "@absl//absl/base:nullability",
        "@absl//absl/container:flat_hash_map",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "pointer_nullability_matchers",
    srcs = ["pointer_nullability_matchers.cc"],
    hdrs = ["pointer_nullability_matchers.h"],
    visibility = ["//nullability/test:__pkg__"],
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
        ":type_nullability",
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
        ":pointer_nullability",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        ":type_nullability",
        "@absl//absl/base:nullability",
        "@absl//absl/log:check",
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
        "@absl//absl/base:nullability",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
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
        ":pointer_nullability",
        ":pointer_nullability_lattice",
        ":pointer_nullability_matchers",
        ":type_nullability",
        "@absl//absl/base:nullability",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:basic",
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
        "@absl//absl/base:nullability",
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
        "@absl//absl/base:nullability",
        "@absl//absl/log:check",
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
        ":type_nullability",
        "@absl//absl/log:check",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "pragma",
    srcs = ["pragma.cc"],
    hdrs = ["pragma.h"],
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
        "@absl//absl/base:nullability",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
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
        "//third_party/benchmark",
        "@absl//absl/base:nullability",
        "@absl//absl/log:check",
        "@absl//absl/strings",
        "@absl//absl/strings:string_view",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:testing",
    ],
)
