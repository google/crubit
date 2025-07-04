"""C++ libraries that help work with Rust types."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
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
        "@googletest//:gtest_main",
    ],
)

cc_library(
    name = "cpp_waker",
    srcs = ["cpp_waker.cc"],
    hdrs = ["cpp_waker.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

rust_library(
    name = "dyn_future",
    srcs = ["dyn_future.rs"],
    aspect_hints = ["//features:supported"],
    cc_deps = [
        ":cpp_waker",
    ],
    visibility = ["//visibility:public"],
)

cc_bindings_from_rust(
    name = "dyn_future_cc",
    compatible_with = ["//buildenv/target:non_prod"],
    crate = ":dyn_future",
    visibility = ["//visibility:public"],
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
        "@fuzztest",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

cc_library(
    name = "str_ref",
    hdrs = ["str_ref.h"],
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
        ":slice_ref",
        "//support/internal:bindings_support",
        "//support/rs_std/internal:is_utf8",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

crubit_cc_test(
    name = "str_ref_test",
    srcs = ["str_ref_test.cc"],
    deps = [
        ":str_ref",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@fuzztest",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)
