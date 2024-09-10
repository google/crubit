# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/bazel_support:toolchain.bzl", "grte_toolchain")
load(
    "//rs_bindings_from_cc/bazel_support:toolchain_headers.bzl",
    "bindings_for_toolchain_headers",
)
load("//support/cc_std:grte_versions.bzl", "CPU", "CPU_GRTE_VERSIONS")

package(default_applicable_licenses = ["//:license"])

filegroup(
    name = "extra_rs_srcs_for_cc_std",
    srcs = glob(
        [
            "*.rs",
        ],
        exclude = [
            "cpp_std_string.rs",
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

config_setting(
    name = "cpu_arm",
    constraint_values = [
        "//third_party/bazel_platforms/cpu:aarch64",
    ],
    visibility = [
        "//visibility:public",
    ],
)

config_setting(
    name = "cpu_x86",
    constraint_values = [
        "//third_party/bazel_platforms/cpu:x86_64",
    ],
    visibility = [
        "//visibility:public",
    ],
)

[
    grte_toolchain(
        name = "grte_{grte}_cpu_{cpu}_toolchain".format(
            cpu = cpu,
            grte = grte,
        ),
        grte_headers = ["//third_party/grte/{grte}_{cpu}/release/usr/grte/{grte}:compile".format(
            cpu = cpu,
            grte = grte,
        )],
    )
    for cpu in CPU
    for grte in CPU_GRTE_VERSIONS[cpu]
]

# TODO(b/336981839): Delete this, it's only here for compatibility.
# Once we delete the `hdrs` parameter to bindings_for_toolchain_headers, this has no purpose.
filegroup(
    name = "empty_filegroup",
    srcs = [],
    visibility = ["//visibility:public"],
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

rust_library(
    name = "vector",
    srcs = ["vector.rs"],
    visibility = [
        "//visibility:public",
    ],
)
