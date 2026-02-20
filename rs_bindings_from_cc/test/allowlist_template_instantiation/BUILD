load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "string_view_test_lib",
    hdrs = ["string_view_test_lib.h"],
    aspect_hints = [
        "//features:unsafe_view",
    ],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":string_view_test_lib",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
