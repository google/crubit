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
    name = "stdlib",
    testonly = 1,
    srcs = ["stdlib.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "stdlib_cc_api",
    testonly = 1,
    crate = ":stdlib",
)

golden_test(
    name = "stdlib_golden_test",
    basename = "stdlib",
    golden_h = "stdlib_cc_api.h",
    golden_rs = "stdlib_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "stdlib",
)

crubit_cc_test(
    name = "stdlib_test",
    srcs = ["stdlib_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): The generated rs_std.h declares `stdin()`, `stdout()` and
    # `stderr()`, which the Windows UCRT defines as macros.
    tags = ["no_windows"],
    deps = [
        ":stdlib_cc_api",
        "//support/rs_std",
        "//support/rs_std:iterator_adapter",
        "//support/rs_std:rs_core",
        "@googletest//:gtest_main",
    ],
)
