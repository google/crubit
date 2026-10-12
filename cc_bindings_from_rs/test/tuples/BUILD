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
    name = "tuples",
    srcs = ["tuples.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "tuples_golden_test",
    basename = "tuples",
    golden_h = "tuples_cc_api.h",
    golden_rs = "tuples_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "tuples",
)

cpp_api_from_rust(
    name = "tuples_cc_api",
    testonly = 1,
    crate = ":tuples",
)

crubit_cc_test(
    name = "tuples_test",
    srcs = ["tuples_test.cc"],
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    deps = [
        ":tuples_cc_api",
        "//support/rs_std:rs_alloc",
        "//testing/base/public:gunit_main",
    ],
)
