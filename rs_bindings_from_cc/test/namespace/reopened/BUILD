load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "reopened_namespace",
    hdrs = ["reopened_namespace.h"],
    deps = [":original_namespace"],
)

crubit_test_cc_library(
    name = "original_namespace",
    hdrs = ["original_namespace.h"],
    # disable Crubit
    aspect_hints = [],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":reopened_namespace"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "//third_party/gtest_rust/googletest",
    ],
)
