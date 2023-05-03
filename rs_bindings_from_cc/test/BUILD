load("@bazel_skylib//:bzl_library.bzl", "bzl_library")

package(default_applicable_licenses = ["//:license"])

sh_test(
    name = "rs_bindings_from_cc_test",
    srcs = ["rs_bindings_from_cc_test.sh"],
    data = ["//rs_bindings_from_cc"],
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
    ],
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
