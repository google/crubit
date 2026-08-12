load("@rules_cc//cc:cc_library.bzl", "cc_library")

package(
    default_applicable_licenses = ["//:license"],
    default_testonly = 1,
    default_visibility = ["//rs_bindings_from_cc/test/bazel/deps_with_same_names:__subpackages__"],
)

cc_library(
    name = "foo",
    srcs = ["foo.cc"],
    hdrs = ["foo.h"],
    aspect_hints = ["//features:supported"],
)
