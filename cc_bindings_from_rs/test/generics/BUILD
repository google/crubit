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
    name = "generics",
    srcs = ["generics.rs"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "generics_golden_test",
    basename = "generics",
    golden_h = "generics_cc_api.h",
    golden_rs = "generics_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "generics",
)

cc_bindings_from_rust(
    name = "generics_cc_api",
    testonly = 1,
    crate = ":generics",
)

crubit_cc_test(
    name = "generics_test",
    srcs = ["generics_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":generics_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
