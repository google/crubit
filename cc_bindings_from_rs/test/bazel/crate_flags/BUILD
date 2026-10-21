load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "crate_flags",
    testonly = 1,
    srcs = ["crate_flags.rs"],
    edition = "2024",
    rustc_flags = [
        # Features specified in rustc_flags should also propagate to Crubit,
        # not only the ones specified in `crate_features`.
        "--cfg=feature=\"required_feature\"",
        # -Dunsafe-code is fine on the target, and should not cause bindings to fail.
        "-Dunsafe-code",
    ],
)

cpp_api_from_rust(
    name = "crate_flags_cc_api",
    testonly = 1,
    crate = ":crate_flags",
)

crubit_cc_test(
    name = "crate_flags_test",
    srcs = ["crate_flags_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":crate_flags_cc_api",
        "@googletest//:gtest_main",
    ],
)
