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
        "//support:annotations",
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
        "//support:annotations",
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
        ":slice_ref",
        "//support:annotations",
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "str_box",
    hdrs = ["str_box.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        ":str_ref",
        "//support:annotations",
        "//support:annotations_internal",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
        "//support/rs_std/internal:is_utf8",
    ],
)

crubit_cc_test(
    name = "box_test",
    srcs = ["box_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        ":box",
        ":pin_box",
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "//support/internal:bindings_support",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "slice_box_test",
    srcs = ["slice_box_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        ":slice_box",
        ":slice_ref",
        "//support:unsafe_relocate_tag",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "str_box_test",
    srcs = ["str_box_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        ":str_box",
        ":str_ref",
        "//support:movable",
        "//support:unsafe_relocate_tag",
        "@googletest//:gtest_main",
    ],
)

# Re-exports of the targets in //support/rs_std.

cc_library(
    name = "char",
    hdrs = ["char.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:char"],
)

alias(
    name = "default_stdlib",
    actual = "//support/rs_std:default_stdlib",
    visibility = ["//visibility:public"],
)

cc_library(
    name = "dyn_callable",
    hdrs = ["dyn_callable.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:dyn_callable"],
)

alias(
    name = "dyn_callable_rs",
    actual = "//support/rs_std:dyn_callable_rs",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

alias(
    name = "dyn_erased_future",
    actual = "//support/rs_std:dyn_erased_future",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
)

cc_library(
    name = "dyn_erased_future_cc",
    hdrs = ["dyn_erased_future.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = ["//support/rs_std:dyn_erased_future_cc"],
)

cc_library(
    name = "enum",
    hdrs = ["enum.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:enum"],
)

alias(
    name = "erased_future",
    actual = "//support/rs_std:erased_future",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
)

cc_library(
    name = "fn",
    hdrs = ["fn.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:fn"],
)

cc_library(
    name = "fn_ref",
    hdrs = ["fn_ref.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:fn_ref"],
)

cc_library(
    name = "int",
    hdrs = ["int.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:int"],
)

cc_library(
    name = "iterator_adapter",
    hdrs = ["iterator_adapter.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:iterator_adapter"],
)

alias(
    name = "lossy_formatter",
    actual = "//support/rs_std:lossy_formatter",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "lossy_formatter_for_bindings",
    hdrs = ["lossy_formatter_for_bindings.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
    deps = ["//support/rs_std:lossy_formatter_for_bindings"],
)

alias(
    name = "lossy_formatter_for_bindings_rs",
    actual = "//support/rs_std:lossy_formatter_for_bindings_rs",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
)

cc_library(
    name = "option",
    hdrs = ["option.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:option"],
)

cc_library(
    name = "result",
    hdrs = ["result.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:result"],
)

alias(
    name = "rs_alloc",
    actual = "//support/rs_std:rs_alloc",
    visibility = ["//visibility:public"],
)

alias(
    name = "rs_core",
    actual = "//support/rs_std:rs_core",
    visibility = ["//visibility:public"],
)

alias(
    name = "rs_std",
    actual = "//support/rs_std:rs_std",
    visibility = ["//visibility:public"],
)

cc_library(
    name = "slice_ref",
    hdrs = ["slice_ref.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:slice_ref"],
)

cc_library(
    name = "str_ref",
    hdrs = ["str_ref.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:str_ref"],
)

cc_library(
    name = "traits",
    hdrs = ["traits.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:traits"],
)

cc_library(
    name = "tuple",
    hdrs = ["tuple.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:tuple"],
)

cc_library(
    name = "unit",
    hdrs = ["unit.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:unit"],
)

cc_library(
    name = "vec",
    hdrs = ["vec.h"],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = ["//support/rs_std:vec"],
)

cc_library(
    name = "waker",
    hdrs = ["waker.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = ["//support/rs_std:waker"],
)
