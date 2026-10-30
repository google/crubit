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
    name = "trait_definition",
    testonly = 1,
    srcs = ["trait_definition.rs"],
    edition = "2024",
)

cpp_api_from_rust(
    name = "trait_definition_cc_api",
    testonly = 1,
    crate = ":trait_definition",
)

rust_library(
    name = "trait_impl",
    testonly = 1,
    srcs = ["trait_impl.rs"],
    edition = "2024",
    deps = [":trait_definition"],
)

cpp_api_from_rust(
    name = "traits_impl_cc_api",
    testonly = 1,
    crate = ":trait_impl",
)

golden_test(
    name = "trait_definition_golden_test",
    basename = "trait_definition",
    golden_h = "trait_definition_cc_api.h",
    golden_rs = "trait_definition_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "trait_definition",
)

golden_test(
    name = "trait_impl_golden_test",
    basename = "trait_impl",
    golden_h = "trait_impl_cc_api.h",
    golden_rs = "trait_impl_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
    ],
    rust_library = "trait_impl",
)

crubit_cc_test(
    name = "trait_impl_test",
    srcs = ["trait_impl_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":traits_impl_cc_api",
        "@googletest//:gtest_main",
    ],
)
