load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "golden_test")

package(
    default_visibility = ["//:__subpackages__"],
)

licenses(["notice"])

crubit_test_cc_library(
    name = "operators_index",
    hdrs = ["operators_index.h"],
)

crubit_rust_test(
    name = "operators_index_test",
    srcs = ["operators_index_test.rs"],
    cc_deps = [
        ":operators_index",
    ],
    deps = [
        "//support:ctor",
        "//support:operator",
        "@crate_index//:googletest",
    ],
)

golden_test(
    name = "operators_index_golden_test",
    basename = "operators_index",
    cc_library = "operators_index",
    golden_cc = "operators_index_rs_api_impl.cc",
    golden_rs = "operators_index_rs_api.rs",
)
