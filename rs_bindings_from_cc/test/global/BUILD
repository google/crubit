load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "global",
    srcs = [
        "global.cc",
    ],
    hdrs = ["global.h"],
)

golden_test(
    name = "global_golden_test",
    basename = "global",
    cc_library = "global",
    golden_cc = "global_api_impl.cc",
    golden_rs = "global_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":global",
    ],
    deps = [
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
