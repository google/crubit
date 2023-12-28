load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "use_forward_declared_template",
    hdrs = ["use_forward_declared_template.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test_forward_declaration.rs"],
    cc_deps = [
        ":use_forward_declared_template",
    ],
)
