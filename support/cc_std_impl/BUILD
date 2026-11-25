# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(default_applicable_licenses = ["//:license"])

_EXTRA_RS_SRCS_FOR_CC_STD = [
    "lossy_utf8.rs",
    "slice_ptr.rs",
]

_CRUBIT_INTERNAL_HELPER_HEADERS = [
    "allocator.h",
    "conversion_function_helpers.h",
    "macos_sdk_workarounds.h",
]

_MANUALLY_BRIDGED_TYPES = [
    "allocator.rs",
    "nullable.rs",
    "optional.rs",
    "shared_ptr.rs",
    "string.rs",
    "string_view.rs",
    "unique_ptr.rs",
    "variant.rs",
    "variant_coproduct.rs",
    "variant_enums.rs",
    "variant_internal.rs",
    "vector.rs",
    "vector_partial_eq.rs",
]

filegroup(
    name = "extra_rs_srcs_for_cc_std",
    srcs = _EXTRA_RS_SRCS_FOR_CC_STD,
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)

filegroup(
    name = "crubit_internal_helper_headers",
    srcs = _CRUBIT_INTERNAL_HELPER_HEADERS,
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "cpp_std_allocator",
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    visibility = [
        "//support/cc_std:__subpackages__",
        "//third_party/crosstool/rust/stable/crubit/support/cc_std:__pkg__",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "manually_bridged_types",
    srcs = _MANUALLY_BRIDGED_TYPES,
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    namespace_path = "std",
    visibility = [
        "//support/cc_std:__subpackages__",
    ],
)
