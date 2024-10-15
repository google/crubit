# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:toolchain_headers.bzl",
    "bindings_for_toolchain_headers",
)

package(default_applicable_licenses = ["//:license"])

filegroup(
    name = "extra_rs_srcs_for_cc_std",
    srcs = glob(
        [
            "*.rs",
        ],
        exclude = [
            "cpp_std_string.rs",
            "cpp_std_allocator.rs",
            "vector.rs",
        ],
    ),
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
    "coroutine",
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
    "vector",
]

LIBC_HEADERS = []

filegroup(
    name = "empty_filegroup",
    srcs = [],
    visibility = ["//visibility:public"],
)

filegroup(
    name = "crubit_internal_helper_headers",
    srcs = ["std_allocator.h"],
    visibility = [
        "//visibility:public",
    ],
)

bindings_for_toolchain_headers(
    name = "cc_std",
    extra_rs_srcs = [":extra_rs_srcs_for_cc_std"],
    public_libc_hdrs = LIBC_HEADERS,
    public_libcxx_hdrs = LIBCXX_HEADERS,
    visibility = [
        "//visibility:public",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "cpp_std_string_rust",
    srcs = ["cpp_std_string.rs"],
)

cc_library(
    name = "cpp_std_string",
    hdrs = ["cpp_std_string.h"],
    aspect_hints = [
        "//features:experimental",
        ":cpp_std_string_rust",
    ],
    visibility = [
        "//visibility:public",
    ],
)

cc_library(
    name = "std_allocator",
    hdrs = ["std_allocator.h"],
    aspect_hints = [
        "//features:supported",
    ],
    visibility = [
        "//visibility:public",
    ],
)

rust_library(
    name = "cpp_std_allocator",
    srcs = ["cpp_std_allocator.rs"],
    cc_deps = [":std_allocator"],
    visibility = [
        "//visibility:public",
    ],
)

rust_library(
    name = "vector",
    srcs = ["vector.rs"],
    visibility = [
        "//visibility:public",
    ],
    deps = [":cpp_std_allocator"],
)
