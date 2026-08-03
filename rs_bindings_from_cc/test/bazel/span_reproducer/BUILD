# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Regression test for absl::Span crate name hardcoding.
#
# Historically, the Crubit generator hardcoded the name of the `absl::Span`
# bindings crate as `::span`. This caused issues when:
# 1. Crate renaming was enabled (which mangles the crate name to its label-encoded version).
# 2. A user-defined Rust library also named `span` was present in the dependency
#    graph (e.g. via `additional_rust_srcs`), causing `::span` to resolve to the
#    wrong crate.
#
# This test verifies that we can compile bindings that use `absl::Span` even when
# a conflicting user-defined `span` crate is present in the dependencies,
# provided that crate renaming is enabled.

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(
    default_testonly = 1,
)

config_setting(
    name = "use_label_encoded_names_for_deps_enabled",
    flag_values = {
        "//common/bazel_support:use_label_encoded_names_for_deps": "True",
    },
)

_compatible_with_flag = select({
    ":use_label_encoded_names_for_deps_enabled": [],
    "//conditions:default": ["//third_party/bazel_platforms:incompatible"],
})

_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

rust_library(
    name = "span",
    srcs = ["rust_span.rs"],
    tags = _tags,
)

additional_rust_srcs_for_crubit_bindings(
    name = "extra_srcs",
    srcs = ["extra.rs"],
    deps = [":span"],
)

cc_library(
    name = "span_usage_cc",
    hdrs = ["span_usage.h"],
    aspect_hints = [
        ":extra_srcs",
        "//features:supported",
    ],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "@abseil-cpp//absl/types:span",
    ],
)

rust_library(
    name = "span_usage_rust",
    srcs = ["span_usage.rs"],
    cc_deps = [":span_usage_cc"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)
