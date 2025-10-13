load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_cli_flag_aspect_hint.bzl",
    "cc_bindings_from_rust_cli_flag",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

cc_bindings_from_rust_cli_flag(
    name = "source_crate_name_core",
    flags = "--source-crate-name=core",
)

rust_library(
    name = "core_proxy",
    srcs = ["empty.rs"],
    aspect_hints = [
        ":source_crate_name_core",
    ],
)

cc_bindings_from_rust(
    name = "core_cc_api",
    testonly = 1,
    crate = ":core_proxy",
)

crubit_cc_test(
    name = "core_bindings_test",
    srcs = ["core_bindings_test.cc"],
    deps = [
        ":core_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
