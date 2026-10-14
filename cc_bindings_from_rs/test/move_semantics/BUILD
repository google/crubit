"""End-to-end tests of `cc_bindings_from_rs`, focusing on correct move semantics."""

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
    name = "move",
    srcs = ["move.rs"],
    edition = "2024",
)

golden_test(
    name = "move_golden_test",
    basename = "move",
    golden_h = "move_cc_api.h",
    golden_rs = "move_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "move",
)

cpp_api_from_rust(
    name = "move_cc_api",
    crate = ":move",
)

crubit_cc_test(
    name = "move_test",
    srcs = ["move_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":move_cc_api",
        "//support:movable",
        "//support/internal:bindings_support",
        "//testing/base/public:gunit_main",
    ],
)
