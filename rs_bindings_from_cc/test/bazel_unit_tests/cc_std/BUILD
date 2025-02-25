load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(":cc_std_test.bzl", "cc_std_test")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "foo",
    hdrs = ["foo.h"],
    deps = [":bar"],
)

crubit_test_cc_library(
    name = "bar",
    hdrs = ["bar.h"],
)

cc_std_test(
    name = "library_without_deps_test",
    dep = ":bar",
)

cc_std_test(
    name = "library_with_deps_test",
    dep = ":foo",
)

sh_binary(
    name = "check_include_directives",
    srcs = ["check_include_directives.sh"],
)

bzl_library(
    name = "cc_std_test_bzl",
    srcs = ["cc_std_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
