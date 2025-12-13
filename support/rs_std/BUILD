"""C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "char",
    srcs = ["char.cc"],
    hdrs = ["char.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],

    # It is important to be thoughtful when adding new dependencies for `char`
    # (and possibly other targets in this BUILD file).  Using mature Abseil APIs
    # seems okay - we should be able to assume that Crubit users have a version
    # of Abseil that is relatively recent (although we can't rely on an
    # exact version and/or exact absl/base/options.h).
    deps = [
        ":str_ref",
        "//support:annotations",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/log:check",
        "@abseil-cpp//absl/strings:string_view",
        "@abseil-cpp//absl/types:span",
    ],
)

alias(
    name = "rs_char",
    actual = "//support/rs_std:char",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "char_test",
    srcs = ["char_test.cc"],
    deps = [
        ":char",
        ":str_ref",
        "//support/rs_std/internal:is_utf8",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
        "@fuzztest",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
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

cc_library(
    name = "slice_ref",
    hdrs = ["slice_ref.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        "//support:annotations_internal",
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
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        ":slice_ref",
        "//support:annotations_internal",
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

rust_library(
    name = "dyn_callable_rs",
    srcs = ["dyn_callable.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:bridge_rust",
    ],
)

cc_library(
    name = "dyn_callable",
    hdrs = ["dyn_callable.h"],
    aspect_hints = [
        "//features:experimental",
        "//features:wrapper",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:bridge_cpp",
        "@abseil-cpp//absl/base:core_headers",
        "@abseil-cpp//absl/base:nullability",
    ],
)

crubit_cc_test(
    name = "dyn_callable_test",
    srcs = ["dyn_callable_test.cc"],
    deps = [
        ":dyn_callable",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)
