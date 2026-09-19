load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "failed_template_instantiation",
    hdrs = ["failed_template_instantiation.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_test_cc_library(
    name = "repeatedly_failed_template_instantiation",
    hdrs = ["repeatedly_failed_template_instantiation.h"],
    aspect_hints = ["//features:template_instantiation"],
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

crubit_rust_test(
    name = "test_repeated",
    srcs = ["test_repeated.rs"],
    cc_deps = [
        ":repeatedly_failed_template_instantiation",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
