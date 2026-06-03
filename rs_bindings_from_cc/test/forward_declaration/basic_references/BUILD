load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration",
    srcs = [
        "forward_declaration.cc",
    ],
    hdrs = ["forward_declaration.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":forward_declaration",
    ],
    deps = [
        "//support:cref",
        "@crate_index//:googletest",
    ],
)

golden_test(
    name = "forward_declaration_golden_test",
    basename = "forward_declaration",
    cc_library = "forward_declaration",
    golden_cc = "forward_declaration_api_impl.cc",
    golden_rs = "forward_declaration_rs_api.rs",
)
