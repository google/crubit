"""End-to-end tests of `cc_bindings_from_rs`, focusing on struct-related
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
    name = "structs",
    testonly = 1,
    srcs = ["structs.rs"],
    edition = "2024",
    proc_macro_deps = ["//support:crubit_annotate"],
)

golden_test(
    name = "structs_golden_test",
    basename = "structs",
    golden_h = "structs_cc_api.h",
    golden_rs = "structs_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "structs",
)

cpp_api_from_rust(
    name = "structs_cc_api",
    testonly = 1,
    crate = ":structs",
)

crubit_cc_test(
    name = "structs_test",
    srcs = ["structs_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":structs_cc_api",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/strings",
    ],
)
