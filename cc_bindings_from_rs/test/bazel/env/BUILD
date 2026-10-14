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
    name = "env",
    testonly = 1,
    srcs = ["env.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "env_cc_api",
    testonly = 1,
    crate = ":env",
)

crubit_cc_test(
    name = "env_test",
    srcs = ["env_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":env_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
