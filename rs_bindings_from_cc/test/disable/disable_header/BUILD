load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_lib",
    hdrs = [
        "disabled_header.h",
        "header.h",
    ],
    aspect_hints = [
        "//features:supported",
    ],
)

crubit_test_cc_library(
    name = "test_lib2",
    hdrs = ["header2.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [":test_lib"],
)

crubit_rust_test(
    name = "disable_header",
    srcs = ["test.rs"],
    cc_deps = [
        ":test_lib",
        ":test_lib2",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
