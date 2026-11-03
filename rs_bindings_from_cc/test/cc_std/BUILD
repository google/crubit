load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        "//support:ctor",
        "//support/cc_std",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "check_cc_std_in_deps",
    srcs = ["test.rs"],
    deps = [
        "//support:ctor",
        "//support/cc_std",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
