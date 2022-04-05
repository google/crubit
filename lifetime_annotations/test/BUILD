"""Test utilities for lifetime_analysis."""

licenses(["notice"])

package(default_visibility = [
    "//lifetime_analysis:__subpackages__",
    "//third_party/crubit:__subpackages__",
])

cc_library(
    name = "named_func_lifetimes",
    testonly = 1,
    srcs = ["named_func_lifetimes.cc"],
    hdrs = ["named_func_lifetimes.h"],
    deps = [
        "//testing/base/public:gunit_for_library_testonly",
        "//lifetime_annotations:lifetime_symbol_table",
        "//lifetime_annotations:type_lifetimes",
        "@llvm///llvm:Support",
    ],
)

cc_library(
    name = "run_on_code",
    testonly = 1,
    srcs = ["run_on_code.cc"],
    hdrs = ["run_on_code.h"],
    deps = [
        "//lifetime_annotations",
        "@llvm///clang:ast",
        "@llvm///clang:tooling",
        "@llvm///llvm:Support",
    ],
)
