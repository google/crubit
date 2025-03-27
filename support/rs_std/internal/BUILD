"""Internal C++ libraries that help work with Rust types."""

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "is_utf8",
    hdrs = ["is_utf8.h"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_test(
    name = "is_utf8_test",
    srcs = ["is_utf8_test.cc"],
    deps = [
        ":is_utf8",
        "//util/textprogressbar",
        "//util/utf8/public:unilib",
        "@abseil-cpp//absl/flags:flag",
        "@abseil-cpp//absl/strings:string_view",
        "@com_google_fuzztest//:fuzztest",
        "@com_google_fuzztest//:fuzztest_gtest_main",
        "@com_google_googletest//:gmock",
    ],
)
