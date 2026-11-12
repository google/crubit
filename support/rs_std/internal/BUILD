"""Internal C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "enum",
    hdrs = ["enum.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

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
    # Depends on //util/terminal, which needs the POSIX `termios.h`.
    tags = [
        "no_android",
        "no_ios",
        "no_windows",
    ],
    deps = [
        ":is_utf8",
        "@abseil-cpp//absl/strings:string_view",
        "@googletest//:gtest_main",
    ],
)
