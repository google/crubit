load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "overloaded",
    hdrs = ["overloaded.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":overloaded"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
