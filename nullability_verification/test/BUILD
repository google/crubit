# Integration tests for nullability verification.

package(default_applicable_licenses = ["//third_party/crubit:license"])

cc_test(
    name = "pointer_nullability_verification_test",
    srcs = ["pointer_nullability_verification_test.cc"],
    deps = [
        "//nullability_verification:pointer_nullability_analysis",
        "//nullability_verification:pointer_nullability_diagnosis",
        "@llvm-project//clang:basic",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)
