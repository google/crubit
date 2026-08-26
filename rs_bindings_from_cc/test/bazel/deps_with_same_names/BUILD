# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
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
    # "//conditions:default": ["//third_party/bazel_platforms:incompatible"],
    "//conditions:default": [],
})

# Hardcoded tags to skip these targets in TAP/Guitar/Builders by default.
# To be removed when the feature is enabled by default.
_tags = [
    "notap",
    "noguitar",
    "manual",
    "nobuilder",
]

# Direct dependency collision test
cc_library(
    name = "lib_direct",
    hdrs = ["direct.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/B_cc_library:a",
    ],
)

rust_library(
    name = "lib_with_two_direct_deps_with_same_name",
    srcs = ["direct.rs"],
    cc_deps = [":lib_direct"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# Indirect dependency collision test
#
# Intentionally uses a symbol from a header only provided indirectly to
# test the Crubit generated bindings. We tag it "nofixdeps" to skip
# build_cleaner.
cc_library(
    name = "lib_indirect",
    hdrs = ["indirect.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags + ["nofixdeps"],
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/C_cc_library_deps_DE:a",
    ],
)

rust_library(
    name = "lib_with_indirect_deps_with_same_names",
    srcs = ["indirect.rs"],
    cc_deps = [":lib_indirect"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# Proto dependency collision test
cc_library(
    name = "lib_proto_deps",
    hdrs = ["proto_deps.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/X_cc_library_deps_YZ:a",
    ],
)

rust_library(
    name = "lib_with_proto_deps_with_same_names",
    srcs = ["proto_deps.rs"],
    cc_deps = [":lib_proto_deps"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# cc_public_library dependency collision test
cc_library(
    name = "lib_public_deps",
    hdrs = ["public_deps.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/I_cc_library_deps_GH:a",
    ],
)

rust_library(
    name = "lib_with_public_deps_with_same_names",
    srcs = ["public_deps.rs"],
    cc_deps = [":lib_public_deps"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# Mixed dependency collision test (additional_rust_srcs + regular C++ deps)
cc_library(
    name = "lib_mixed_deps",
    hdrs = ["mixed_deps.h"],
    aspect_hints = ["//features:supported"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/J_cc_library_deps_AB:a",
    ],
)

rust_library(
    name = "lib_with_mixed_deps",
    srcs = ["mixed_deps.rs"],
    cc_deps = [":lib_mixed_deps"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# Aspect hint dep test (cc_deps in aspect hint matches one of client's deps)
cc_library(
    name = "lib_aspect_hint_dep",
    hdrs = ["aspect_hint_dep.h"],
    aspect_hints = [
        ":aspect_hint_dep_extra_srcs",
        "//features:supported",
    ],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/A_cc_library:a",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/B_cc_library:a",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "aspect_hint_dep_extra_srcs",
    srcs = ["aspect_hint_dep_extra.rs"],
    cc_deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/deps/A_cc_library:a",
    ],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

rust_library(
    name = "lib_with_aspect_hint_dep",
    srcs = ["aspect_hint_dep.rs"],
    cc_deps = [":lib_aspect_hint_dep"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

# rust_api_from_cpp test: we have pair of a client and dep cc libraries,
# together with corresponding rust_api_from_cpp that depend on each other.
# The manual bindings of the client library refer to the bindings of the
# dependency.
cc_library(
    name = "cc_deps_test_dep",
    hdrs = ["cc_deps_test_dep.h"],
    aspect_hints = [":cc_deps_test_dep_rust.hint"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

rust_api_from_cpp(
    name = "cc_deps_test_dep_rust",
    cpp_target = ":cc_deps_test_dep",
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

cc_library(
    name = "cc_deps_test_lib",
    hdrs = ["cc_deps_test_lib.h"],
    aspect_hints = [":cc_deps_test_lib_rust.hint"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
)

rust_api_from_cpp(
    name = "cc_deps_test_lib_rust",
    srcs = ["cc_deps_test_lib.rs"],
    cpp_target = ":cc_deps_test_lib",
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        ":cc_deps_test_dep_rust",
    ],
)

rust_test(
    name = "cc_deps_test",
    srcs = ["cc_deps_test.rs"],
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    deps = [
        ":cc_deps_test_lib_rust",
        "@crate_index//:googletest",
    ],
)

build_test(
    name = "build_test",
    tags = _tags,
    target_compatible_with = _compatible_with_flag,
    targets = [
        ":lib_with_two_direct_deps_with_same_name",
        ":lib_with_indirect_deps_with_same_names",
        ":lib_with_proto_deps_with_same_names",
        ":lib_with_public_deps_with_same_names",
        ":lib_with_mixed_deps",
        ":lib_with_aspect_hint_dep",
        ":cc_deps_test",
    ],
)
