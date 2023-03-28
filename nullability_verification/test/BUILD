# Integration tests for nullability verification.

package(default_applicable_licenses = ["//third_party/crubit:license"])

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
