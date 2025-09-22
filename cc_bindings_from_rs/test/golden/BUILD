load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

# These are needed by golden_test, which can be run in other packages.
exports_files([
    "test.sh",
    "LICENSE_HEADER",
])

TESTS = [name[:-3] for name in glob(
    ["*.rs"],
    exclude = ["*cc_api_impl.rs"],
)]

[rust_library(
    name = name + "_rust",
    srcs = [name + ".rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support:bridge_rust",
    ],
) for name in TESTS]

[golden_test(
    name = name + "_test",
    basename = name,
    golden_h = name + "_cc_api.h",
    golden_rs = name + "_cc_api_impl.rs",
    rust_library = name + "_rust",
) for name in TESTS]

[cc_bindings_from_rust(
    name = name + "_cc_api",
    testonly = 1,
    crate = ":" + name + "_rust",
) for name in TESTS]

[crubit_cc_test(
    name = name + "_cc_test",
    srcs = ["empty_cc_test.cc"],
    deps = ["%s_cc_api" % name],
) for name in TESTS]

bzl_library(
    name = "golden_test",
    srcs = ["golden_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule_bzl",
    ],
)
