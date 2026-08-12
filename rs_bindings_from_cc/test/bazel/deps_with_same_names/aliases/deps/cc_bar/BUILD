load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)

package(
    default_applicable_licenses = ["//:license"],
    default_testonly = 1,
    default_visibility = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__"],
)

cc_library(
    name = "bar",
    srcs = ["bar.cc"],
    hdrs = ["bar.h"],
    aspect_hints = [":bar_rust.hint"],
)

rust_api_from_cpp(
    name = "bar_rust",
    cpp_target = ":bar",
)
