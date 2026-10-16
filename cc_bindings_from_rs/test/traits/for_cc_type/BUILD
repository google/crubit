load(
    "@rules_cc//cc:cc_library.bzl",
    "cc_library",
)
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_type",
    hdrs = ["cc_type.h"],
    aspect_hints = [":cc_type_rust.hint"],
)

rust_api_from_cpp(
    name = "cc_type_rust",
    testonly = 1,
    cpp_target = ":cc_type",
)

rust_library(
    name = "trait",
    testonly = 1,
    srcs = ["trait.rs"],
    edition = "2024",
    deps = [":cc_type_rust"],
)

cpp_api_from_rust(
    name = "trait_cc_api",
    testonly = 1,
    crate = ":trait",
)

golden_test(
    name = "trait_golden_test",
    basename = "trait",
    golden_h = "trait_cc_api.h",
    golden_rs = "trait_cc_api_impl.rs",
    rust_library = "trait",
)

crubit_cc_test(
    name = "trait_test",
    srcs = ["trait_test.cc"],
    deps = [
        ":cc_type",
        ":trait_cc_api",
        "@googletest//:gtest_main",
    ],
)
