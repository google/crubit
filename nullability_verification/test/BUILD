# Integration tests for nullability verification.

package(default_applicable_licenses = ["//:license"])

cc_test(
    name = "pointer_nullability_verification_test",
    srcs = ["pointer_nullability_verification_test.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_library(
    name = "check_diagnostics",
    testonly = 1,
    srcs = ["check_diagnostics.cc"],
    hdrs = ["check_diagnostics.h"],
    deps = [
        "//nullability_verification:pointer_nullability_analysis",
        "//nullability_verification:pointer_nullability_diagnosis",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//third-party/unittest:gtest",
    ],
)

cc_test(
    name = "assert_nullability_test",
    srcs = ["assert_nullability_test.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "basic",
    srcs = ["basic.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "binary_ops",
    srcs = ["binary_ops.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "casts",
    srcs = ["casts.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "comparisons",
    srcs = ["comparisons.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "constructors",
    srcs = ["constructors.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "fields",
    srcs = ["fields.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "function_calls",
    srcs = ["function_calls.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "initialization",
    srcs = ["initialization.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "merge",
    srcs = ["merge.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "return_statements",
    srcs = ["return_statements.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "templates",
    srcs = ["templates.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "this_pointer",
    srcs = ["this_pointer.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "variance",
    srcs = ["variance.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)
