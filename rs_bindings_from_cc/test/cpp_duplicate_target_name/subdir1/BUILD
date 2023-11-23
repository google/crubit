load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "A",
    hdrs = ["subdir1_A.h"],
    visibility = [
        "//rs_bindings_from_cc/test/cpp_duplicate_target_name:__subpackages__",
    ],
)
