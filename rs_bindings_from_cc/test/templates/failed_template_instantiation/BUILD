load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "failed_template_instantiation",
    hdrs = ["failed_template_instantiation.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":failed_template_instantiation",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
