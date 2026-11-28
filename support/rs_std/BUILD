"""C++ libraries that help work with Rust types."""

load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cpp_api_from_rust_toolchain_bindings",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

exports_files(
    [
        "char.cc",
        "char.h",
        "dyn_callable.h",
        "dyn_erased_future.h",
        "enum.h",
        "fn.h",
        "fn_ref.h",
        "int.h",
        "iterator_adapter.h",
        "lossy_formatter_for_bindings.h",
        "never.h",
        "option.h",
        "result.h",
        "slice_ref.h",
        "str_ref.h",
        "traits.h",
        "tuple.h",
        "unit.h",
        "vec.h",
        "waker.cc",
        "waker.h",
    ],
    visibility = ["//support/rs:__pkg__"],
)

alias(
    name = "char",
    actual = "//support/rs:char",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:char.
    tags = [
        "alt_dep=//support/rs:char",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "int",
    actual = "//support/rs:int",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:int.
    tags = [
        "alt_dep=//support/rs:int",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "int_test",
    srcs = ["int_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:int",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "rs_char",
    actual = "//support/rs:char",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:char.
    tags = [
        "alt_dep=//support/rs:char",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "char_test",
    srcs = ["char_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:char",
        "//support/rs:str_ref",
        "//support/rs_std/internal:is_utf8",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/types:span",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "waker",
    actual = "//support/rs:waker",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:waker.
    tags = [
        "alt_dep=//support/rs:waker",
        "avoid_dep",
    ],
    visibility = [
        "//:__subpackages__",
    ],
)

rust_library(
    name = "erased_future",
    srcs = ["erased_future.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2024",
    # Prefer the re-export in //support/rs.
    tags = [
        "alt_dep=//support/rs:erased_future",
        "avoid_dep",
    ],
    visibility = [
        "//:__subpackages__",
    ],
)

crubit_rust_test(
    name = "erased_future_test",
    size = "small",
    srcs = ["erased_future_test.rs"],
    deps = [
        "//support/rs:erased_future",
        "@crate_index//:googletest",
    ],
)

rust_library(
    name = "dyn_erased_future",
    srcs = ["dyn_erased_future.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2024",
    # Prefer the re-export in //support/rs.
    tags = [
        "alt_dep=//support/rs:dyn_erased_future",
        "avoid_dep",
    ],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        "//support:crubit_support",
    ],
)

alias(
    name = "dyn_erased_future_cc",
    actual = "//support/rs:dyn_erased_future_cc",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:dyn_erased_future_cc.
    tags = [
        "alt_dep=//support/rs:dyn_erased_future_cc",
        "avoid_dep",
    ],
    visibility = [
        "//:__subpackages__",
    ],
)

alias(
    name = "slice_ref",
    actual = "//support/rs:slice_ref",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:slice_ref.
    tags = [
        "alt_dep=//support/rs:slice_ref",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

crubit_cc_test(
    name = "slice_ref_test",
    srcs = ["slice_ref_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:slice_ref",
        "@abseil-cpp//absl/types:span",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "str_ref",
    actual = "//support/rs:str_ref",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:str_ref.
    tags = [
        "alt_dep=//support/rs:str_ref",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

crubit_cc_test(
    name = "str_ref_test",
    srcs = ["str_ref_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:str_ref",
        "@abseil-cpp//absl/strings",
        "@abseil-cpp//absl/strings:string_view",
        "@googletest//:gtest_main",
    ],
)

rust_library(
    name = "dyn_callable_rs",
    srcs = ["dyn_callable.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2024",
    # Prefer the re-export in //support/rs.
    tags = [
        "alt_dep=//support/rs:dyn_callable_rs",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//support:bridge_rust",
    ],
)

alias(
    name = "dyn_callable",
    actual = "//support/rs:dyn_callable",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:dyn_callable.
    tags = [
        "alt_dep=//support/rs:dyn_callable",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

crubit_cc_test(
    name = "dyn_callable_test",
    srcs = ["dyn_callable_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:dyn_callable",
        "@abseil-cpp//absl/functional:any_invocable",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "fn_ref",
    actual = "//support/rs:fn_ref",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:fn_ref.
    tags = [
        "alt_dep=//support/rs:fn_ref",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

crubit_cc_test(
    name = "fn_ref_test",
    srcs = ["fn_ref_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:fn_ref",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "fn",
    actual = "//support/rs:fn",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:fn.
    tags = [
        "alt_dep=//support/rs:fn",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

crubit_cc_test(
    name = "fn_test",
    srcs = ["fn_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support:movable",
        "//support/rs:fn",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "traits",
    actual = "//support/rs:traits",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:traits.
    tags = [
        "alt_dep=//support/rs:traits",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "iterator_adapter",
    actual = "//support/rs:iterator_adapter",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:iterator_adapter.
    tags = [
        "alt_dep=//support/rs:iterator_adapter",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "iterator_adapter_test",
    srcs = ["iterator_adapter_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:iterator_adapter",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "option",
    actual = "//support/rs:option",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:option.
    tags = [
        "alt_dep=//support/rs:option",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "result",
    actual = "//support/rs:result",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:result.
    tags = [
        "alt_dep=//support/rs:result",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "vec",
    actual = "//support/rs:vec",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:vec.
    tags = [
        "alt_dep=//support/rs:vec",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "tuple",
    actual = "//support/rs:tuple",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:tuple.
    tags = [
        "alt_dep=//support/rs:tuple",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

alias(
    name = "never",
    actual = "//support/rs:never",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:never.
    tags = [
        "alt_dep=//support/rs:never",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "never_test",
    srcs = ["never_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:never",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "unit",
    actual = "//support/rs:unit",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:unit.
    tags = [
        "alt_dep=//support/rs:unit",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "unit_test",
    srcs = ["unit_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:unit",
        "@googletest//:gtest_main",
    ],
)

alias(
    name = "enum",
    actual = "//support/rs:enum",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:enum.
    tags = [
        "alt_dep=//support/rs:enum",
        "avoid_dep",
    ],
    visibility = [
        "//visibility:public",
    ],
)

crubit_cc_test(
    name = "enum_tag_encoding_test",
    srcs = ["enum_tag_encoding_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:enum",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "enum_storage_test",
    srcs = ["enum_storage_test.cc"],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:enum",
        "@googletest//:gtest_main",
    ],
)

crubit_cc_test(
    name = "enum_test",
    srcs = [
        "enum_test.cc",
        "enum_test_option_i32.h",
    ],
    tags = [
        "no_android",
        "no_ios",
    ],
    deps = [
        "//support/rs:enum",
        "@abseil-cpp//absl/base:core_headers",
        "@googletest//:gtest_main",
    ],
)

rust_library(
    name = "lossy_formatter",
    srcs = ["lossy_formatter.rs"],
    compatible_with = ["//buildenv/target:non_prod"],
    edition = "2024",
    # Prefer the re-export in //support/rs.
    tags = [
        "alt_dep=//support/rs:lossy_formatter",
        "avoid_dep",
    ],
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
    edition = "2024",
    # Prefer the re-export in //support/rs.
    tags = [
        "alt_dep=//support/rs:lossy_formatter_for_bindings_rs",
        "avoid_dep",
    ],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":lossy_formatter",  # build_cleaner: keep
        "//support/ffi_11",
    ],
)

alias(
    name = "lossy_formatter_for_bindings",
    actual = "//support/rs:lossy_formatter_for_bindings",
    compatible_with = ["//buildenv/target:non_prod"],
    # Prefer //support/rs:lossy_formatter_for_bindings.
    tags = [
        "alt_dep=//support/rs:lossy_formatter_for_bindings",
        "avoid_dep",
    ],
    visibility = ["//:__subpackages__"],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_core",
    compatible_with = ["//buildenv/target:non_prod"],
    cpp_namespace = "rs::core",
    crate_name = "core",
    # Prefer //support/rs:rs_core.
    tags = [
        "alt_dep=//support/rs:rs_core",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_alloc",
    compatible_with = ["//buildenv/target:non_prod"],
    cpp_namespace = "rs::alloc",
    crate_name = "alloc",
    # Prefer //support/rs:rs_alloc.
    tags = [
        "alt_dep=//support/rs:rs_alloc",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
    deps = [":rs_core"],
)

cpp_api_from_rust_toolchain_bindings(
    name = "rs_std",
    compatible_with = ["//buildenv/target:non_prod"],
    cpp_namespace = "rs::std",
    crate_name = "std",
    # Prefer //support/rs:rs_std.
    tags = [
        "alt_dep=//support/rs:rs_std",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
    deps = [
        ":rs_alloc",
        ":rs_core",
    ],
)

alias(
    name = "default_stdlib",
    actual = select({
        "@platforms//os:none": ":rs_core",
        "@rules_rust//:is_no_std": ":rs_alloc",
        "//conditions:default": ":rs_std",
    }),
    # Prefer //support/rs:default_stdlib.
    tags = [
        "alt_dep=//support/rs:default_stdlib",
        "avoid_dep",
    ],
    visibility = ["//visibility:public"],
)

label_flag(
    name = "stdlib",
    build_setting_default = ":default_stdlib",
    visibility = ["//visibility:public"],
)
