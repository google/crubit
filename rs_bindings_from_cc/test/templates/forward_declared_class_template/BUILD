load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "use_forward_declared_template",
    hdrs = ["use_forward_declared_template.h"],
    tags = [
        # b/274663418: Remove when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":use_forward_declared_template",
    ],
    tags = [
        # b/274663418: Remove when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
)
