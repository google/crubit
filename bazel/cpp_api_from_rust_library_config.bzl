# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Re-exports cc_bindings_from_rust_library_config as cpp_api_from_rust_library_config."""

load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl",
    _cc_bindings_from_rust_library_config = "cc_bindings_from_rust_library_config",
)

cpp_api_from_rust_library_config = _cc_bindings_from_rust_library_config
