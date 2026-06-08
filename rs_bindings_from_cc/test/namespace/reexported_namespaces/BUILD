load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_cli_flag_aspect_hint.bzl",
    "rust_bindings_from_cc_cli_flag",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

rust_bindings_from_cc_cli_flag(
    name = "reexport_namespaces_aspect_hint",
    flags = "--reexported_namespaces=absl,base",
)

crubit_test_cc_library(
    name = "reexported_namespaces",
    hdrs = ["reexported_namespaces.h"],
    aspect_hints = [
        ":reexport_namespaces_aspect_hint",
        "//features:supported",
    ],
)

golden_test(
    name = "reexported_namespaces_golden_test",
    basename = "reexported_namespaces",
    cc_library = ":reexported_namespaces",
    golden_cc = "reexported_namespaces_api_impl.cc",
    golden_rs = "reexported_namespaces_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":reexported_namespaces",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
