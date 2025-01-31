"""C++ libraries that help work with Rust types."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "rs_char",
    hdrs = ["rs_char.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    visibility = [
        "//visibility:public",
    ],

    # It is important to be thoughtful when adding new dependencies for `char`
    # (and possibly other targets in this BUILD file).  Using mature Abseil APIs
    # seems okay - we should be able to assume that Crubit users have a version
    # of Abseil that is relatively recent (although we can't rely on an
    # exact version and/or exact absl/base/options.h).
    deps = [
        "//support/internal:bindings_support",
        "@abseil-cpp//absl/base:core_headers",
    ],
)

crubit_cc_test(
    name = "rs_char_test",
    srcs = ["rs_char_test.cc"],
    deps = [
        ":rs_char",
        "@com_google_googletest//:gtest_main",
    ],
)

cc_library(
    name = "slice_ref",
    hdrs = ["slice_ref.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    visibility = [
        "//visibility:public",
    ],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        "//support/internal:bindings_support",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/types:span",
    ],
)

crubit_cc_test(
    name = "slice_ref_test",
    srcs = ["slice_ref_test.cc"],
    deps = [
        ":slice_ref",
        "@abseil-cpp//absl/types:span",
        "@com_google_fuzztest//:fuzztest",
        "@com_google_fuzztest//:fuzztest_gtest_main",
        "@com_google_googletest//:gmock",
    ],
)
