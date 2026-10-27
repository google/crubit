"""C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "box",
    hdrs = ["box.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "pin_box",
    hdrs = ["pin_box.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "slice_box",
    hdrs = ["slice_box.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
        "//support/rs_std:slice_ref",
    ],
)

cc_library(
    name = "str_box",
    hdrs = ["str_box.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
        "//support/rs_std:str_ref",
        "//support/rs_std/internal:is_utf8",
    ],
)

crubit_cc_test(
    name = "box_test",
    srcs = ["box_test.cc"],
    deps = [
        ":box",
        ":pin_box",
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gtest",
    ],
)

crubit_cc_test(
    name = "slice_box_test",
    srcs = ["slice_box_test.cc"],
    deps = [
        ":slice_box",
        "//support:unsafe_relocate_tag",
        "//support/rs_std:slice_ref",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gtest",
    ],
)

crubit_cc_test(
    name = "str_box_test",
    srcs = ["str_box_test.cc"],
    deps = [
        ":str_box",
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/rs_std:str_ref",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gtest",
    ],
)
