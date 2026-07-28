"""Internal C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "is_utf8",
    hdrs = ["is_utf8.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

crubit_cc_test(
    name = "is_utf8_test",
    srcs = ["is_utf8_test.cc"],
    deps = [
        ":is_utf8",
        "//util/textprogressbar",
        "//util/utf8/public:unilib",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/strings:string_view",
        "@fuzztest",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)
