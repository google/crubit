# Integration tests for nullability verification.

load("//nullability/test:nullability_test.bzl", "nullability_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "check_diagnostics",
    testonly = 1,
    srcs = ["check_diagnostics.cc"],
    hdrs = ["check_diagnostics.h"],
    deps = [
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_diagnosis",
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

cc_binary(
    name = "nullability_test",
    testonly = 1,
    srcs = ["nullability_test.cc"],
    deps = [
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:type_nullability",
        "@absl//absl/log:check",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:Support",
    ],
)

sh_test(
    name = "nullability_test_test",
    srcs = ["nullability_test_test.sh"],
    args = ["$(location :nullability_test)"],
    data = [":nullability_test"],
)

cc_test(
    name = "arrays",
    srcs = ["arrays.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "aliases",
    srcs = ["aliases.cc"],
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

nullability_test(
    name = "binary_ops",
    srcs = ["binary_ops.cc"],
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

nullability_test(
    name = "comparisons",
    srcs = ["comparisons.cc"],
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
    name = "function_pointers",
    srcs = ["function_pointers.cc"],
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
        "//nullability:pointer_nullability_analysis",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "operator_new",
    srcs = ["operator_new.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "parens",
    srcs = ["parens.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "pointer_arithmetic",
    srcs = ["pointer_arithmetic.cc"],
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

nullability_test(
    name = "symbolic_nullability",
    srcs = ["symbolic_nullability.cc"],
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
    name = "temporary_materialization",
    srcs = ["temporary_materialization.cc"],
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

nullability_test(
    name = "types",
    srcs = ["types.cc"],
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
