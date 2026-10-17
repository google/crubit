"""Tests that Crubit bindings build when C++ modules are partially or fully disabled.

The generated `_rust_api_impl.cc` is compiled with the wrapped target's copts and may consume
precompiled header modules. These targets turn various module features off to make sure that
doing so never breaks the build.
"""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "dep_lib",
    hdrs = ["dep_lib.h"],
)

crubit_test_cc_library(
    name = "fno_modules_lib",
    srcs = ["fno_modules_lib.cc"],
    hdrs = ["fno_modules_lib.h"],
    copts = ["-fno-modules"],
    features = ["-use_header_modules"],
    deps = [
        ":dep_lib",
    ],
)

# `-fno-modules` without `-use_header_modules`. The target itself has no srcs, but the generated
# `_rust_api_impl.cc` is compiled with `-fno-modules` and must not load `.pcm` files.
crubit_test_cc_library(
    name = "fno_modules_only_lib",
    hdrs = ["fno_modules_only_lib.h"],
    copts = ["-fno-modules"],
    deps = [":dep_lib"],
)

crubit_test_cc_library(
    name = "no_use_header_modules_lib",
    hdrs = ["no_use_header_modules_lib.h"],
    features = ["-use_header_modules"],
    deps = [":dep_lib"],
)

crubit_test_cc_library(
    name = "no_module_maps_lib",
    hdrs = ["no_module_maps_lib.h"],
    features = ["-module_maps"],
    deps = [":dep_lib"],
)

crubit_test_cc_library(
    name = "no_header_modules_lib",
    hdrs = ["no_header_modules_lib.h"],
    features = ["-header_modules"],
    deps = [":dep_lib"],
)

crubit_test_cc_library(
    name = "uses_fno_modules_lib",
    hdrs = ["uses_fno_modules_lib.h"],
    deps = [
        ":dep_lib",
        ":fno_modules_lib",
    ],
)

crubit_rust_test(
    name = "modules_off_test",
    srcs = ["modules_off_test.rs"],
    cc_deps = [
        ":dep_lib",
        ":fno_modules_lib",
        ":fno_modules_only_lib",
        ":no_header_modules_lib",
        ":no_module_maps_lib",
        ":no_use_header_modules_lib",
        ":uses_fno_modules_lib",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

# `cpp_srcs` are concatenated into the generated `_rust_api_impl.cc`. They were never checked by
# `layering_check`, so that compile stays textual (no module maps) when `cpp_srcs` are present.
cc_library(
    name = "cpp_srcs_lib",
    hdrs = ["cpp_srcs_lib.h"],
    aspect_hints = [":cpp_srcs_lib_rust.hint"],
    deps = [":uses_fno_modules_lib"],
)

rust_api_from_cpp(
    name = "cpp_srcs_lib_rust",
    srcs = ["cpp_srcs_extra.rs"],
    cpp_srcs = ["cpp_srcs_extra.cc"],
    cpp_target = ":cpp_srcs_lib",
)

crubit_rust_test(
    name = "cpp_srcs_test",
    srcs = ["cpp_srcs_test.rs"],
    deps = [
        ":cpp_srcs_lib_rust",
        "@crate_index//:googletest",
    ],
)
