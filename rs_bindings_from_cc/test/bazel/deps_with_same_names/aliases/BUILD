# Test cases about usage of aliases in manual bindings attached to C++ libraries.

load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

package(
    default_applicable_licenses = ["//:license"],
    default_testonly = 1,
    default_visibility = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__"],
)

# Test case 1: rust_api_from_cpp with aliases
cc_library(
    name = "key_cc",
    srcs = ["key.cc"],
    hdrs = ["key.h"],
    aspect_hints = [":key_rust.hint"],
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_bar:bar",  # buildcleaner: keep
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_foo:foo",  # buildcleaner: keep
    ],
)

rust_api_from_cpp(
    name = "key_rust",
    srcs = ["key.rs"],
    aliases = {
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_bar:bar_rust": "aliased_bar",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_foo:foo": "aliased_foo",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/rust_foo:foo": "aliased_rust_foo",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/foo:foo": "aliased_rust_foo_sub",
    },
    cpp_target = ":key_cc",
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/foo",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/rust_foo:foo",
    ],
)

# Test case 2: additional_rust_srcs_for_crubit_bindings with aliases
cc_library(
    name = "key2_cc",
    srcs = ["key2.cc"],
    hdrs = ["key2.h"],
    aspect_hints = [":key2_hint"],
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_bar:bar",  # buildcleaner: keep
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_foo:foo",  # buildcleaner: keep
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "key2_hint",
    srcs = ["key2_hint.rs"],
    aliases = {
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_bar:bar_rust": "aliased_bar",
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/cc_foo:foo": "aliased_foo",
    },
    deps = [
        "//rs_bindings_from_cc/test/bazel/deps_with_same_names/aliases/deps/rust_foo:foo",
    ],
)

rust_library(
    name = "key2_client",
    srcs = ["key2_client.rs"],
    aliases = {
        ":key2_cc": "key2_cc_rust",
    },
    cc_deps = [":key2_cc"],
)

build_test(
    name = "build_test",
    targets = [
        ":key_rust",
        ":key2_client",
    ],
)
