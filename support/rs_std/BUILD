"""C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
    "cpp_api_from_rust_toolchain_bindings",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "char",
    srcs = ["char.cc"],
    hdrs = ["char.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],

    # It is important to be thoughtful when adding new dependencies for `char`
    # (and possibly other targets in this BUILD file).  Using mature Abseil APIs
    # seems okay - we should be able to assume that Crubit users have a version
    # of Abseil that is relatively recent (although we can't rely on an
    # exact version and/or exact absl/base/options.h).
    deps = [
        ":str_ref",
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "int",
    hdrs = ["int.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        "//support:annotations",
    ],
)

crubit_cc_test(
    name = "int_test",
    srcs = ["int_test.cc"],
    deps = [
        ":int",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
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
    name = "waker",
    srcs = ["waker.cc"],
    hdrs = ["waker.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
)

rust_library(
    name = "erased_future",
    srcs = ["erased_future.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
)

crubit_rust_test(
    name = "erased_future_test",
    size = "small",
    srcs = ["erased_future_test.rs"],
    deps = [
        ":erased_future",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "dyn_erased_future",
    srcs = ["dyn_erased_future.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    link_deps = [
        ":waker",
    ],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":erased_future",
    ],
)

cc_library(
    name = "dyn_erased_future_cc",
    hdrs = ["dyn_erased_future.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":dyn_erased_future",  # build_cleaner: keep, defines extern "C" functions we declare.
        ":waker",
    ],
)

cc_library(
    name = "slice_ref",
    hdrs = ["slice_ref.h"],
    # Enable bidirectional bindings (via crubit_internal_rust_type).
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

crubit_cc_test(
    name = "slice_ref_test",
    srcs = ["slice_ref_test.cc"],
    # Continuous fuzzing (autofuzz) is not supported for Crubit targets because crubit_cc_test uses
    # a custom transition to the LLVM unstable toolchain.
    tags = ["noautofuzz"],
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
    visibility = ["//visibility:public"],

    # Try to avoid unnecessary dependencies, so that Crubit users have to pull in as little as
    # possible. Using mature Abseil APIs seems okay - we should be able to assume that Crubit users
    # have a version of Abseil that is relatively recent (although we can't rely on an exact version
    # and/or exact absl/base/options.h).
    deps = [
        ":slice_ref",
        "//support:annotations_internal",
        "//support/internal:bindings_support",
        "//support/rs_std/internal:is_utf8",
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
        "@abseil-cpp//absl/base:nullability",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

crubit_cc_test(
    name = "dyn_callable_test",
    srcs = ["dyn_callable_test.cc"],
    deps = [
        ":dyn_callable",
        "@abseil-cpp//absl/functional:any_invocable",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

cc_library(
    name = "fn_ref",
    hdrs = ["fn_ref.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [],
)

crubit_cc_test(
    name = "fn_ref_test",
    srcs = ["fn_ref_test.cc"],
    deps = [
        ":fn_ref",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

cc_library(
    name = "fn",
    hdrs = ["fn.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//visibility:public"],
    deps = [],
)

crubit_cc_test(
    name = "fn_test",
    srcs = ["fn_test.cc"],
    deps = [
        ":fn",
        "//support:movable",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

cc_library(
    name = "traits",
    hdrs = ["traits.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

cc_library(
    name = "iterator_adapter",
    hdrs = ["iterator_adapter.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":traits",
    ],
)

crubit_cc_test(
    name = "iterator_adapter_test",
    srcs = ["iterator_adapter_test.cc"],
    deps = [
        ":iterator_adapter",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

cc_library(
    name = "option",
    hdrs = ["option.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":unit",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "result",
    hdrs = ["result.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [
        ":unit",
        "//support/internal:bindings_support",
    ],
)

cc_library(
    name = "vec",
    hdrs = ["vec.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

cc_library(
    name = "tuple",
    hdrs = ["tuple.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

cc_library(
    name = "unit",
    hdrs = ["unit.h"],
    aspect_hints = ["//features:experimental"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
    deps = [],
)

crubit_cc_test(
    name = "unit_test",
    srcs = ["unit_test.cc"],
    deps = [
        ":unit",
        "@fuzztest//:fuzztest_gtest_main",
        "@googletest//:gmock",
    ],
)

rust_library(
    name = "lossy_formatter",
    srcs = ["lossy_formatter.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//visibility:public",
    ],
)

crubit_rust_test(
    name = "lossy_formatter_test",
    crate = ":lossy_formatter",
    deps = [
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "lossy_formatter_for_bindings_rs",
    srcs = ["lossy_formatter_for_bindings.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":lossy_formatter",
        "//support/ffi_11",
    ],
)

cc_library(
    name = "lossy_formatter_for_bindings",
    hdrs = ["lossy_formatter_for_bindings.h"],
    compatible_with = ["//buildenv/target:non_prod"],
    visibility = ["//:__subpackages__"],
    deps = [
        # We depend on the extern "C" symbols in lossy_formatter_for_bindings_rs.
        ":lossy_formatter_for_bindings_rs",
    ],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_core",
    cpp_namespace = "rs::core",
    crate_name = "core",
    visibility = ["//visibility:public"],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_alloc",
    cpp_namespace = "rs::alloc",
    crate_name = "alloc",
    visibility = ["//visibility:public"],
    deps = [":rs_core"],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_std",
    cpp_namespace = "rs::std",
    crate_name = "std",
    visibility = ["//visibility:public"],
    deps = [
        ":rs_alloc",
        ":rs_core",
    ],
)

crubit_cc_test(
    name = "include_cpp_config_test",
    srcs = ["include_cpp_config_test.cc"],
    deps = [
        "//support/public:cpp_config",
    ],
)

cc_test(
    name = "include_cpp_config_test_stable",
    srcs = ["include_cpp_config_test.cc"],
    deps = [
        "//support/public:cpp_config",
    ],
)
