"""End-to-end tests of `cc_bindings_from_rs`, focusing on function-related
bindings."""

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
    name = "functions",
    testonly = 1,
    srcs = ["functions.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [
        "//support:ctor",
    ],
)

cpp_api_from_rust(
    name = "functions_cc_api",
    testonly = 1,
    crate = ":functions",
)

golden_test(
    name = "functions_golden_test",
    basename = "functions",
    golden_h = "functions_cc_api.h",
    golden_rs = "functions_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "functions",
)

crubit_cc_test(
    name = "functions_test",
    srcs = ["functions_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":functions_cc_api",
        "//support/rs_std:char",
        "//testing/base/public:gunit_main",
    ],
)
