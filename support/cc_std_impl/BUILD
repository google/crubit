# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(default_applicable_licenses = ["//:license"])

filegroup(
    name = "extra_rs_srcs_for_cc_std",
    srcs = ["slice_ptr.rs"],
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)

filegroup(
    name = "crubit_internal_helper_headers",
    srcs = [
        "conversion_function_helpers.h",
        "std_allocator.h",
    ],
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "cpp_std_allocator",
    srcs = [
        "cpp_std_allocator.rs",
    ],
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    namespace_path = "crubit_cc_std_internal::std_allocator",
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "manually_bridged_types",
    srcs = [
        "string.rs",
        "string_view.rs",
        "unique_ptr.rs",
        "vector.rs",
        "vector_partial_eq.rs",
    ],
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    namespace_path = "std",
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)
