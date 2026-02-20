"""End-to-end tests of `cc_bindings_from_rs`, focusing on enums-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "foo",
    testonly = 1,
    srcs = ["foo.rs"],
)

rust_library(
    name = "bar",
    testonly = 1,
    srcs = ["bar.rs"],
)

rust_library(
    name = "foobar",
    testonly = 1,
    srcs = ["foobar.rs"],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        ":bar",
        ":foo",
    ],
)

cc_bindings_from_rust(
    name = "foobar_cc_api",
    testonly = 1,
    crate = ":foobar",
)

crubit_cc_test(
    name = "option_specialization_diamond",
    srcs = ["foobar_test.cc"],
    deps = [
        ":foobar_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
