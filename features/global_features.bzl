# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A shared list of supported features, including fine grained additions to `:supported`."""

visibility(["//..."])

SUPPORTED_FEATURES = [
    "supported",
    "assume_lifetimes",
    "template_instantiation",
    "proto_references",
    "layout_compat_optional",
    "always_specialize_generics_in_cpp_api_from_rust",
    "forward_declare",
    "iterator_item_references",
    "nonnull_smart_pointers",
    "reserve_standard_macros",
    "infer_send_sync",
]

# Per-feature denylists live in features/denylists/supported_feature_denylists.bzl, not here, so
# that editing them only affects one package instead of every package that loads this file.
