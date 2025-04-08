"""Test utilities for lifetime_analysis."""

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//:__subpackages__"],
)

cc_library(
    name = "named_func_lifetimes",
    testonly = 1,
    srcs = ["named_func_lifetimes.cc"],
    hdrs = ["named_func_lifetimes.h"],
    deps = [
        "//lifetime_annotations:lifetime",
        "//lifetime_annotations:lifetime_symbol_table",
        "//lifetime_annotations:type_lifetimes",
        "@googletest//:gtest",
        "@llvm-project//llvm:Support",
    ],
)

cc_library(
    name = "run_on_code",
    testonly = 1,
    srcs = ["run_on_code.cc"],
    hdrs = ["run_on_code.h"],
    deps = [
        "//lifetime_annotations",
        "@llvm-project//clang:ast",
        "@llvm-project//clang:frontend",
        "@llvm-project//clang:serialization",
        "@llvm-project//clang:tooling",
        "@llvm-project//llvm:Support",
    ],
)
