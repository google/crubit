"""End-to-end tests of `cc_bindings_from_rs`, focusing on
module/namespace-related bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "modules",
    testonly = 1,
    srcs = ["modules.rs"],
    edition = "2024",
)

golden_test(
    name = "modules_golden_test",
    basename = "modules",
    golden_h = "modules_cc_api.h",
    golden_rs = "modules_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "modules",
)

cpp_api_from_rust(
    name = "modules_cc_api",
    testonly = 1,
    crate = ":modules",
)

crubit_cc_test(
    name = "modules_test",
    srcs = ["modules_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":modules_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
