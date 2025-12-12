load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "global_lib",
    srcs = [
        "global.cc",
    ],
    hdrs = ["global.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":global_lib",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
