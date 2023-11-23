load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "header",
    hdrs = ["header.h"],
    deps = [
        "//rs_bindings_from_cc/test/cpp_duplicate_target_name/subdir1:A",
        # subdir2:B depends on subdir2:A (both of which don't have Crubit enabled), but that's
        # sufficient to cause a conflict with subdir1:A in Crubit-generated code.
        "//rs_bindings_from_cc/test/cpp_duplicate_target_name/subdir2:B",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = [
        "test.rs",
    ],
    cc_deps = [
        ":header",
    ],
    tags = [
        # b/312679890: It currently fails to build: multiple candidates for `rmeta` dependency `A` found
        "manual",
        "notap",
        "nobuilder",
    ],
)
