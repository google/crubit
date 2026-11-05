load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "rs_bindings_from_cc_test",
    srcs = ["rs_bindings_from_cc_test.rs"],
    data = [
        "//rs_bindings_from_cc",
    ],
    rustc_env = {
        "RS_BINDINGS_FROM_CC": "$(rootpath //rs_bindings_from_cc)",
    },
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
    deps = [
        "//common:external_binaries",
        "@crate_index//:googletest",
        "@crate_index//:tempfile",  # v3
    ],
)

bzl_library(
    name = "test_bindings",
    srcs = ["test_bindings.bzl"],
    visibility = ["//visibility:private"],
)
