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
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "enums_golden_test",
    basename = "enums",
    golden_h = "enums_cc_api.h",
    golden_rs = "enums_cc_api_impl.rs",
    rust_library = "enums",
)

cc_bindings_from_rust(
    name = "enums_cc_api",
    testonly = 1,
    crate = ":enums",
)

crubit_cc_test(
    name = "enums_test",
    srcs = ["enums_test.cc"],
    deps = [
        ":enums_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "cpp_enums",
    testonly = 1,
    srcs = ["cpp_enums.rs"],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "cpp_enums_golden_test",
    basename = "cpp_enums",
    golden_h = "cpp_enums_cc_api.h",
    rust_library = "cpp_enums",
)

cc_bindings_from_rust(
    name = "cpp_enums_cc_api",
    testonly = 1,
    crate = ":cpp_enums",
)

crubit_cc_test(
    name = "cpp_enums_test",
    srcs = ["cpp_enums_test.cc"],
    deps = [
        ":cpp_enums_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "option",
    testonly = 1,
    srcs = ["option.rs"],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    rustc_flags = ["-Zallow-features=rustc_attrs"],
)

golden_test(
    name = "option_golden_test",
    basename = "option",
    golden_h = "option_cc_api.h",
    golden_rs = "option_cc_api_impl.rs",
    rust_library = "option",
)

cc_bindings_from_rust(
    name = "option_cc_api",
    testonly = 1,
    crate = ":option",
)

crubit_cc_test(
    name = "option_test",
    srcs = ["option_test.cc"],
    deps = [
        ":option_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
