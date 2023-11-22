load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "core",
    hdrs = ["header.h"],
    visibility = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name:__subpackages__",
    ],
    deps = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name/subdir1:core",
    ],
)
