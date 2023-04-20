# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(
    "//rs_bindings_from_cc/bazel_support:toolchain_headers.bzl",
    "bindings_for_toolchain_headers",
)

package(default_applicable_licenses = ["//:license"])

filegroup(
    name = "extra_rs_srcs_for_cc_std",
    srcs = glob([
        "*.rs",
    ]),
)

LIBCXX_HEADERS = [
    "algorithm",
    "any",
    "array",
    "atomic",
    "bitset",
    "cassert",
    "cctype",
    "cerrno",
    "cfenv",
    "cfloat",
    "charconv",
    "chrono",
    "cinttypes",
    "climits",
    "cmath",
    "condition_variable",
    "csetjmp",
    "csignal",
    "cstdarg",
    "cstddef",
    "cstdint",
    "cstdio",
    "cstdlib",
    "cstring",
    "ctime",
    "cuchar",
    "cwchar",
    "cwctype",
    "deque",
    "exception",
    "forward_list",
    "functional",
    "initializer_list",
    "ios",
    "limits",
    "list",
    "map",
    "memory",
    "mutex",
    "new",
    "numeric",
    "optional",
    "queue",
    "ratio",
    "set",
    "scoped_allocator",
    "stack",
    "stdexcept",
    "streambuf",
    "string",
    "string_view",
    "system_error",
    "tuple",
    "type_traits",
    "typeindex",
    "typeinfo",
    "unordered_map",
    "unordered_set",
    "utility",
    "variant",
    # "vector", # TODO(b/248542210): include vector when we can conditionally import methods.
]

LIBC_HEADERS = []

config_setting(
    name = "llvm_unstable",
    values = {
        "crosstool_top": "//nowhere/llvm:everything",
    },
)

filegroup(
    name = "toolchain_headers",
    srcs = select({
               ":llvm_unstable": ["//nowhere/llvm:cc-std-headers"],
               "//conditions:default": ["//nowhere/llvm:cc-std-headers"],
           }) +
           select({
               "@bazel_tools//tools/target_cpu:aarch64-all": ["//third_party/grte/v5_arm/release/usr/grte/v5:compile"],
               "@bazel_tools//tools/target_cpu:x86_64-all": ["//third_party/grte/v5_x86/release/usr/grte/v5:compile"],
           }),
    visibility = ["//visibility:private"],
)

bindings_for_toolchain_headers(
    name = "cc_std",
    hdrs = ":toolchain_headers",
    extra_rs_srcs = [":extra_rs_srcs_for_cc_std"],
    public_libc_hdrs = LIBC_HEADERS,
    public_libcxx_hdrs = LIBCXX_HEADERS,
    visibility = ["//visibility:public"],
)
