load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "conversion_test",
    srcs = ["conversion_test.rs"],
    deps = [
        "//support/ffi_11",
        "@crate_index//:static_assertions",
    ],
)
