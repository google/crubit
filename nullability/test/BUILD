# Integration tests for nullability verification.

load("@bazel_tools//tools/build_defs/cc:cc_embed_data.bzl", "cc_embed_data")
load("//nullability/test:nullability_test.bzl", "nullability_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "check_diagnostics",
    testonly = 1,
    srcs = ["check_diagnostics.cc"],
    hdrs = ["check_diagnostics.h"],
    deps = [
        ":test_headers",
        "//nullability:pointer_nullability_diagnosis",
        "//nullability:pragma",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:ast_matchers",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:testing",
        "@llvm-project//clang:tooling",
        "@llvm-project//clang/unittests:dataflow_testing_support",
        "@llvm-project//llvm:Support",
        "@llvm-project//third-party/unittest:gmock",
        "@llvm-project//third-party/unittest:gtest",
    ],
)

cc_test(
    name = "check_diagnostics_test",
    srcs = ["check_diagnostics_test.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "assert_nullability_test",
    srcs = ["assert_nullability_test.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_binary(
    name = "nullability_test",
    testonly = 1,
    srcs = ["nullability_test.cc"],
    deps = [
        ":test_headers",
        "//nullability:pointer_nullability",
        "//nullability:pointer_nullability_analysis",
        "//nullability:pointer_nullability_lattice",
        "//nullability:pragma",
        "//nullability:type_nullability",
        "@abseil-cpp//absl/log:check",
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
        "//nullability:pointer_nullability_diagnosis",
        "//nullability:pragma",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:basic",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:TestingSupport",
        "@llvm-project//third-party/unittest:gmock",
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
    name = "check_macros",
    srcs = ["check_macros.cc"],
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
    name = "convergence",
    srcs = ["convergence.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_test(
    name = "default_arguments",
    srcs = ["default_arguments.cc"],
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

nullability_test(
    name = "forwarding_functions",
    srcs = ["forwarding_functions.cc"],
)

cc_test(
    name = "forwarding_functions_diagnosis",
    srcs = ["forwarding_functions_diagnosis.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//clang:testing",
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

nullability_test(
    name = "initialization",
    srcs = ["initialization.cc"],
)

cc_test(
    name = "initialization_diagnosis",
    srcs = ["initialization_diagnosis.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

nullability_test(
    name = "join",
    srcs = ["join.cc"],
)

cc_test(
    name = "join_diagnosis",
    srcs = ["join_diagnosis.cc"],
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
    name = "path_sensitive",
    srcs = ["path_sensitive.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

nullability_test(
    name = "pointer_arithmetic",
    srcs = ["pointer_arithmetic.cc"],
)

cc_test(
    name = "pointer_arithmetic_diagnosis",
    srcs = ["pointer_arithmetic_diagnosis.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

nullability_test(
    name = "resugaring",
    srcs = ["resugaring.cc"],
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
    name = "smart_pointers",
    srcs = ["smart_pointers.cc"],
)

cc_test(
    name = "smart_pointers_diagnosis",
    srcs = ["smart_pointers_diagnosis.cc"],
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

nullability_test(
    name = "pragma",
    srcs = [
        "pragma.cc",
        "pragma_none.h",
        "pragma_nonnull.h",
        "pragma_support.h",
    ],
)

cc_test(
    name = "pragma_diagnosis",
    srcs = ["pragma_diagnosis.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)

cc_embed_data(
    name = "test_headers",
    srcs = [
        "nullability_annotations.h",
        "nullability_test.h",
        "check.h",
        "check_diagnostics_preamble.h",
        "new",
        "memory",
        "type_traits",
        "utility",
    ],
    outs = [
        "test_headers.cc",
        "test_headers.h",
        "test_headers.o",
    ],
    embedopts = ["--namespace=clang::tidy::nullability"],
    visibility = ["//nullability:__subpackages__"],
)

cc_test(
    name = "consistent_annotations",
    srcs = ["consistent_annotations.cc"],
    deps = [
        ":check_diagnostics",
        "@llvm-project//third-party/unittest:gtest",
        "@llvm-project//third-party/unittest:gtest_main",
    ],
)
