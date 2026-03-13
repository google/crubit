"""End-to-end tests of `cc_bindings_from_rs`, focusing on use statements."""

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

rust_library(
    name = "transitive",
    testonly = 1,
    srcs = ["transitive.rs"],
)

rust_library(
    name = "direct",
    testonly = 1,
    srcs = ["direct.rs"],
    deps = [":transitive"],
)

rust_library(
    name = "transitive_reexports",
    testonly = 1,
    srcs = ["transitive_reexports.rs"],
    deps = [":direct"],
)

cc_bindings_from_rust(
    name = "transitive_reexports_cc_api",
    testonly = 1,
    crate = ":transitive_reexports",
)

crubit_cc_test(
    name = "transitive_reexports_test",
    srcs = ["transitive_reexports_test.cc"],
    deps = [
        ":transitive_reexports_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

golden_test(
    name = "transitive_reexports_golden_test",
    basename = "transitive_reexports",
    golden_h = "transitive_reexports_cc_api.h",
    golden_rs = "transitive_reexports_cc_api_impl.rs",
    rust_library = "transitive_reexports",
)
