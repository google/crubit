load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

sh_test(
    name = "rs_bindings_from_cc_test",
    srcs = ["rs_bindings_from_cc_test.sh"],
    data = ["//rs_bindings_from_cc"],
    deps = [
        "//util/shell/gbash",
        "//util/shell/gbash:unit",
    ],
)

bzl_library(
    name = "test_bindings",
    srcs = ["test_bindings.bzl"],
    visibility = ["//:__subpackages__"],
)
