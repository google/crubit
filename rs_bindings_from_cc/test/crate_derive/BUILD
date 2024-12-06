load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "crate_derive",
    hdrs = ["crate_derive.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support/internal:bindings_support",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":crate_derive",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",
    ],
)
