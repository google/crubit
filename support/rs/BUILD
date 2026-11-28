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

cc_library(
    name = "char",
    srcs = ["//support/rs_std:char.cc"],
    hdrs = [
        "char.h",
        "//support/rs_std:char.h",
    ],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:supported"],
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
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
    ],
)

alias(
    name = "default_stdlib",
    actual = "//support/rs_std:default_stdlib",
    # Prevent build_cleaner from prompting between :default_stdlib and :rs_std/:rs_alloc/:rs_core.
    tags = ["avoid_dep"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "dyn_callable",
    hdrs = [
        "dyn_callable.h",
        "//support/rs_std:dyn_callable.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:bridge_cpp",
        "//support/internal:cpp_config",
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
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
    hdrs = [
        "dyn_erased_future.h",
        "//support/rs_std:dyn_erased_future.h",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":waker",
        "//support:crubit_support",  # build_cleaner: keep, defines extern "C" functions we declare.
    ],
)

# Support for the C++ bindings of Rust enums: the `rs::` free functions
# (`get_if`, `get_unchecked`, `holds_alternative`, `discriminant`) and the
# `crubit::internal` machinery generated enum classes are built from.
cc_library(
    name = "enum",
    hdrs = [
        "enum.h",
        "//support/rs_std:enum.h",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
        "//support/rs_std/internal:enum",
    ],
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
    hdrs = [
        "fn.h",
        "//support/rs_std:fn.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "fn_ref",
    hdrs = [
        "fn_ref.h",
        "//support/rs_std:fn_ref.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "int",
    hdrs = [
        "int.h",
        "//support/rs_std:int.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations",
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "iterator_adapter",
    hdrs = [
        "iterator_adapter.h",
        "//support/rs_std:iterator_adapter.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        ":traits",
        "//support/internal:cpp_config",
    ],
)

alias(
    name = "lossy_formatter",
    actual = "//support/rs_std:lossy_formatter",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "lossy_formatter_for_bindings",
    hdrs = [
        "lossy_formatter_for_bindings.h",
        "//support/rs_std:lossy_formatter_for_bindings.h",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
    deps = [
        # We depend on the extern "C" symbols in lossy_formatter_for_bindings_rs.
        ":lossy_formatter_for_bindings_rs",  # build_cleaner: keep
    ],
)

alias(
    name = "lossy_formatter_for_bindings_rs",
    actual = "//support/rs_std:lossy_formatter_for_bindings_rs",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
)

cc_library(
    name = "never",
    hdrs = [
        "never.h",
        "//support/rs_std:never.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support:annotations_internal",
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "option",
    hdrs = [
        "option.h",
        "//support/rs_std:option.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        ":never",
        ":unit",
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "result",
    hdrs = [
        "result.h",
        "//support/rs_std:result.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        ":never",
        ":unit",
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
    ],
)

alias(
    name = "rs_alloc",
    actual = "//support/rs_std:rs_alloc",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

alias(
    name = "rs_core",
    actual = "//support/rs_std:rs_core",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

alias(
    name = "rs_std",
    actual = "//support/rs_std:rs_std",
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
)

cc_library(
    name = "slice_ref",
    hdrs = [
        "slice_ref.h",
        "//support/rs_std:slice_ref.h",
    ],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "slice_ref_header_modules_virtual_include",
    testonly = True,
    hdrs = ["slice_ref_header_modules_virtual_include.h"],
    features = ["header_modules"],
    deps = [":slice_ref"],
)

cc_library(
    name = "slice_ref_header_modules_depot_path",
    testonly = True,
    hdrs = ["slice_ref_header_modules_depot_path.h"],
    features = ["header_modules"],
    deps = [":slice_ref"],
)

crubit_cc_test(
    name = "slice_ref_header_modules_odr_test",
    srcs = ["slice_ref_header_modules_odr_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        ":slice_ref_header_modules_depot_path",
        ":slice_ref_header_modules_virtual_include",
        "@googletest//:gtest_main",
    ],
)

cc_library(
    name = "str_ref",
    hdrs = [
        "str_ref.h",
        "//support/rs_std:str_ref.h",
    ],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        ":slice_ref",
        "//support:annotations_internal",
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
        "//support/rs_std/internal:is_utf8",
    ],
)

cc_library(
    name = "traits",
    hdrs = [
        "traits.h",
        "//support/rs_std:traits.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "tuple",
    hdrs = [
        "tuple.h",
        "//support/rs_std:tuple.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "unit",
    hdrs = [
        "unit.h",
        "//support/rs_std:unit.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "vec",
    hdrs = [
        "vec.h",
        "//support/rs_std:vec.h",
    ],
    aspect_hints = ["//features:supported"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [
        "//support/internal:bindings_support",
        "//support/internal:cpp_config",
    ],
)

cc_library(
    name = "waker",
    srcs = ["//support/rs_std:waker.cc"],
    hdrs = [
        "waker.h",
        "//support/rs_std:waker.h",
    ],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "//support/internal:cpp_config",
    ],
)
