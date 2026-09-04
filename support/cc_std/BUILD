# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "//rs_bindings_from_cc/bazel_support:toolchain_headers.bzl",
    "bindings_for_toolchain_headers",
)
load(
    "//support/cc_std_impl:headers.bzl",
    "LIBCXX_HEADERS",
    "LIBC_HEADERS",
)

package(default_applicable_licenses = ["//:license"])

bindings_for_toolchain_headers(
    name = "cc_std",
    compatible_with = [
        "//buildenv/target:non_prod",
    ],
    crate_features = ["len_capacity_encoding"],
    extra_hdrs = ["//support/cc_std_impl:crubit_internal_helper_headers"],
    extra_rs_srcs = [
        "//support/cc_std_impl:extra_rs_srcs_for_cc_std",
        "//support/cc_std_impl:cpp_std_allocator",
        "//support/cc_std_impl:manually_bridged_types",
    ],
    public_libc_hdrs = LIBC_HEADERS,
    public_libcxx_hdrs = LIBCXX_HEADERS,
    visibility = [
        "//visibility:public",
    ],
)
