load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

bzl_library(
    name = "attach_aspect",
    srcs = ["attach_aspect.bzl"],
    visibility = [
        "//cc_bindings_from_rs/test/bazel/unit_tests:__subpackages__",
    ],
)
