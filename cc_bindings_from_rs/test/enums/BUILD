"""End-to-end tests of `cc_bindings_from_rs`, focusing on enums-related
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
    name = "enums",
    testonly = 1,
    srcs = ["enums.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "enums_golden_test",
    basename = "enums",
    golden_h = "enums_cc_api.h",
    golden_rs = "enums_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "enums",
)

cpp_api_from_rust(
    name = "enums_cc_api",
    testonly = 1,
    crate = ":enums",
)

crubit_cc_test(
    name = "enums_test",
    srcs = ["enums_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":enums_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "cpp_enums",
    testonly = 1,
    srcs = ["cpp_enums.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "cpp_enums_golden_test",
    basename = "cpp_enums",
    golden_h = "cpp_enums_cc_api.h",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "cpp_enums",
)

cpp_api_from_rust(
    name = "cpp_enums_cc_api",
    testonly = 1,
    crate = ":cpp_enums",
)

crubit_cc_test(
    name = "cpp_enums_test",
    srcs = ["cpp_enums_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":cpp_enums_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "option",
    testonly = 1,
    srcs = ["option.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "option_golden_test",
    basename = "option",
    golden_h = "option_cc_api.h",
    golden_rs = "option_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "option",
)

cpp_api_from_rust(
    name = "option_cc_api",
    testonly = 1,
    crate = ":option",
)

crubit_cc_test(
    name = "option_test",
    srcs = ["option_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":option_cc_api",
        "//support/rs_std",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "result",
    testonly = 1,
    srcs = ["result.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "result_golden_test",
    basename = "result",
    golden_h = "result_cc_api.h",
    golden_rs = "result_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "result",
)

cpp_api_from_rust(
    name = "result_cc_api",
    testonly = 1,
    crate = ":result",
)

crubit_cc_test(
    name = "result_test",
    srcs = ["result_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":result_cc_api",
        "//support:movable",
        "//testing/base/public:gunit_main",
    ],
)
