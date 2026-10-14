load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

licenses(["notice"])

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
)

rust_library(
    name = "build_rs_user",
    testonly = 1,
    srcs = ["build_rs_user.rs"],
    edition = "2024",
    deps = [":build_script"],
)

cpp_api_from_rust(
    name = "build_rs_user_cc_api",
    testonly = 1,
    crate = ":build_rs_user",
)

crubit_cc_test(
    name = "build_rs_test",
    srcs = ["build_rs_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    deps = [
        ":build_rs_user_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
