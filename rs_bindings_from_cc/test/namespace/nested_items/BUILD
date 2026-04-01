load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nested_items",
    hdrs = ["nested_items.h"],
    deps = ["//support:annotations"],
)

golden_test(
    name = "nested_items_golden_test",
    basename = "nested_items",
    cc_library = ":nested_items",
    golden_cc = "nested_items_api_impl.cc",
    golden_rs = "nested_items_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":nested_items",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
