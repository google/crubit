load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load("@rules_rust//rust:rust_make_no_std.bzl", "rust_make_no_std")
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
    name = "no_std",
    srcs = ["no_std.rs"],
)

golden_test(
    name = "no_std_golden_test",
    basename = "no_std",
    golden_h = "no_std_cc_api.h",
    golden_rs = "no_std_cc_api_impl.rs",
    rust_library = "no_std",
)

rust_make_no_std(
    name = "make_no_std_no_std",
    testonly = 1,
    deps = [":no_std"],
)

cc_bindings_from_rust(
    name = "no_std_cc_api",
    testonly = 1,
    crate = ":no_std",
)

crubit_cc_test(
    name = "no_std_test",
    srcs = ["no_std_test.cc"],
    deps = [
        ":no_std_cc_api",
        "//testing/base/public:gunit_main",
    ],
)

rust_library(
    name = "no_core",
    srcs = ["no_core.rs"],
    rustc_flags = ["-Zallow-features=no_core,profiler_runtime,staged_api"],
)

golden_test(
    name = "no_core_golden_test",
    basename = "no_core",
    golden_h = "no_core_cc_api.h",
    golden_rs = "no_core_cc_api_impl.rs",
    rust_library = "no_core",
)

cc_bindings_from_rust(
    name = "no_core_cc_api",
    testonly = 1,
    crate = ":no_core",
)

crubit_cc_test(
    name = "no_core_test",
    srcs = ["no_core_test.cc"],
    deps = [
        ":no_core_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
